import com.google.firebase.crashlytics.buildtools.gradle.CrashlyticsExtension

plugins {
    id("com.android.application")
    id("org.mozilla.rust-android-gradle.rust-android")
    id("com.google.dagger.hilt.android")
    id("com.google.gms.google-services")
    id("com.google.firebase.crashlytics")
    id("com.diffplug.spotless") version "6.25.0"
    id("com.google.firebase.appdistribution")
    id("kotlin-parcelize")
    id("androidx.navigation.safeargs")

    kotlin("android")
    kotlin("kapt")
}

spotless {
    format("misc") {
        target("*.gradle", "*.md", ".gitignore")
        trimTrailingWhitespace()
        indentWithSpaces()
        endWithNewline()
    }
    kotlin {
        ktlint()
        target("**/*.kt")
        targetExclude("**/generated/*")
        licenseHeader("/* Licensed under Apache 2.0 (C) \$YEAR Firezone, Inc. */", "^(package |import |@file)")
    }
    kotlinGradle {
        ktlint()
        target("**/*.gradle.kts")
        targetExclude("**/generated/*")
    }
}

apply(plugin = "org.mozilla.rust-android-gradle.rust-android")

android {
    buildFeatures {
        buildConfig = true
    }

    namespace = "dev.firezone.android"
    compileSdk = 34

    // Life is easier if we just match the default NDK on the Ubuntu 22.04 runners
    // https://github.com/actions/runner-images/blob/main/images/ubuntu/Ubuntu2204-Readme.md#android
    ndkVersion = "25.2.9519653"

    defaultConfig {
        applicationId = "dev.firezone.android"
        // Android 8
        minSdk = 26
        targetSdk = 34
        versionCode = (System.currentTimeMillis() / 1000 / 10).toInt()
        // mark:next-android-version
        versionName = "1.0.3"
        multiDexEnabled = true
        testInstrumentationRunner = "dev.firezone.android.core.HiltTestRunner"
    }

    signingConfigs {
        create("release") {
            // Find this in the Engineering 1Password vault
            storeFile = file(System.getenv("KEYSTORE_PATH") ?: "keystore.jks")
            keyAlias = "upload"
            storePassword = System.getenv("KEYSTORE_PASSWORD") ?: ""
            keyPassword = System.getenv("KEYSTORE_KEY_PASSWORD") ?: ""
        }
    }

    buildTypes {
        // Debug Config
        getByName("debug") {
            isDebuggable = true
            resValue("string", "app_name", "\"Firezone (Dev)\"")

            buildConfigField("String", "AUTH_BASE_URL", "\"https://app.firez.one\"")
            buildConfigField("String", "API_URL", "\"wss://api.firez.one\"")
            buildConfigField(
                "String",
                "LOG_FILTER",
                "\"connlib_client_android=debug,firezone_tunnel=trace,phoenix_channel=debug,connlib_shared=debug," +
                    "boringtun=debug,snownet=debug,str0m=debug,connlib_client_shared=debug,info\"",
            )
        }

        // Release Config
        getByName("release") {
            signingConfig = signingConfigs.getByName("release")

            // Enables code shrinking, obfuscation, and optimization for only
            // your project's release build type. Make sure to use a build
            // variant with `isDebuggable=false`.
            // Not compatible with Rust
            isMinifyEnabled = false

            // Enables resource shrinking, which is performed by the
            // Android Gradle plugin.
            isShrinkResources = false

            // Includes the default ProGuard rules files that are packaged with
            // the Android Gradle plugin. To learn more, go to the section about
            // R8 configuration files.
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro",
            )
            isDebuggable = false

            configure<CrashlyticsExtension> {
                // Enable processing and uploading of native symbols to Firebase servers.
                // By default, this is disabled to improve build speeds.
                // This flag must be enabled to see properly-symbolicated native
                // stack traces in the Crashlytics dashboard.
                nativeSymbolUploadEnabled = true
                unstrippedNativeLibsDir = layout.buildDirectory.dir("rustJniLibs")
            }

            resValue("string", "app_name", "\"Firezone\"")

            buildConfigField("String", "AUTH_BASE_URL", "\"https://app.firezone.dev\"")
            buildConfigField("String", "API_URL", "\"wss://api.firezone.dev\"")
            buildConfigField("String", "LOG_FILTER", "\"str0m=warn,info\"")
            firebaseAppDistribution {
                serviceCredentialsFile = System.getenv("FIREBASE_CREDENTIALS_PATH")
                artifactType = "AAB"
                releaseNotes = "https://github.com/firezone/firezone/releases"
                groups = "firezone-engineering"
                artifactPath = "app/build/outputs/bundle/release/app-release.aab"
            }
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    buildFeatures {
        viewBinding = true
    }

    testOptions {
        unitTests {
            isIncludeAndroidResources = true
        }
    }
}

dependencies {
    // AndroidX
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.appcompat:appcompat:1.7.0")
    implementation("androidx.preference:preference-ktx:1.2.1")
    implementation("androidx.constraintlayout:constraintlayout:2.1.4")

    // Material
    implementation("com.google.android.material:material:1.11.0")

    // Lifecycle
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.8.1")
    implementation("androidx.lifecycle:lifecycle-extensions:2.2.0")
    implementation("androidx.lifecycle:lifecycle-viewmodel-ktx:2.8.1")
    implementation("androidx.lifecycle:lifecycle-livedata-ktx:2.8.1")

    // Navigation
    implementation("androidx.navigation:navigation-fragment-ktx:2.7.7")
    implementation("androidx.navigation:navigation-ui-ktx:2.7.7")

    // Hilt
    implementation("com.google.dagger:hilt-android:2.51.1")
    implementation("androidx.browser:browser:1.8.0")
    kapt("androidx.hilt:hilt-compiler:1.2.0")
    kapt("com.google.dagger:hilt-android-compiler:2.51.1")
    // Instrumented Tests
    androidTestImplementation("com.google.dagger:hilt-android-testing:2.51.1")
    kaptAndroidTest("com.google.dagger:hilt-android-compiler:2.51.1")
    androidTestImplementation("androidx.test:runner:1.5.2")
    androidTestImplementation("androidx.navigation:navigation-testing:2.7.7")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
    androidTestImplementation("androidx.test.espresso:espresso-contrib:3.5.1")
    androidTestImplementation("androidx.test.uiautomator:uiautomator:2.3.0")
    // Unit Tests
    testImplementation("com.google.dagger:hilt-android-testing:2.51.1")

    // Retrofit 2
    implementation("com.squareup.retrofit2:retrofit:2.11.0")
    implementation("com.squareup.retrofit2:converter-moshi:2.11.0")

    // OkHttp
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
    implementation("com.squareup.okhttp3:logging-interceptor:4.12.0")

    // Moshi
    implementation("com.squareup.moshi:moshi-kotlin:1.15.1")
    implementation("com.squareup.moshi:moshi:1.15.1")

    // Gson
    implementation("com.google.code.gson:gson:2.10.1")

    // Security
    implementation("androidx.security:security-crypto:1.1.0-alpha06")

    // JUnit
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.fragment:fragment-testing:1.6.2")

    // Import the BoM for the Firebase platform
    implementation(platform("com.google.firebase:firebase-bom:33.1.0"))

    // Add the dependencies for the Crashlytics and Analytics libraries
    // When using the BoM, you don't specify versions in Firebase library dependencies
    implementation("com.google.firebase:firebase-crashlytics-ktx")
    implementation("com.google.firebase:firebase-crashlytics-ndk")
    implementation("com.google.firebase:firebase-analytics-ktx")
}

cargo {
    if (gradle.startParameter.taskNames.any { it.lowercase().contains("release") }) {
        profile = "release"
    } else {
        profile = "debug"
    }
    // Needed for Ubuntu 22.04
    pythonCommand = "python3"
    prebuiltToolchains = true
    verbose = true
    module = "../../../rust/connlib/clients/android"
    libname = "connlib"
    targets =
        listOf(
            "arm64",
            "x86_64",
            "x86",
            "arm",
        )
    targetDirectory = "../../../rust/target"
}

tasks.matching { it.name.matches(Regex("merge.*JniLibFolders")) }.configureEach {
    inputs.dir(layout.buildDirectory.file("rustJniLibs/android"))
    dependsOn("cargoBuild")
}
