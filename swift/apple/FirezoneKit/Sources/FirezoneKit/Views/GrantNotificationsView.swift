//
//  GrantNotificationsView.swift
//  (c) 2024 Firezone, Inc.
//  LICENSE: Apache-2.0
//

import Combine
import Foundation
import SwiftUI
import UserNotifications

@MainActor
public final class GrantNotificationsViewModel: ObservableObject {
  private let sessionNotification: SessionNotification
  private let onDecisionChanged: (UNAuthorizationStatus) async -> Void

  init(
    sessionNotification: SessionNotification,
    onDecisionChanged: @escaping (UNAuthorizationStatus) async -> Void
  ) {
    self.sessionNotification = sessionNotification
    self.onDecisionChanged = onDecisionChanged
  }

  func grantNotificationButtonTapped(errorHandler: GlobalErrorHandler) {
    Task.detached { [weak self] in
      guard let self else { return }

      do {
        let decision = try await sessionNotification.askUserForNotificationPermissions()
        await onDecisionChanged(decision)
      } catch {
        Log.error(error)

        await MainActor.run {
          errorHandler.handle(ErrorAlert(
            title: "Error granting notifications",
            error: error
          ))
        }
      }
    }

  }
}

struct GrantNotificationsView: View {
  @ObservedObject var model: GrantNotificationsViewModel
  @EnvironmentObject var errorHandler: GlobalErrorHandler

  public var body: some View {
    VStack(
      alignment: .center,
      content: {
        Spacer()
        Image("LogoText")
          .resizable()
          .scaledToFit()
          .frame(maxWidth: 320)
          .padding(.horizontal, 10)
        Spacer()
        Text(
          "Firezone requires your permission to show local notifications when you need to sign in again."
        )
        .font(.body)
        .multilineTextAlignment(.center)
        .padding(EdgeInsets(top: 0, leading: 5, bottom: 0, trailing: 5))
        Spacer()
        Image(systemName: "bell")
          .imageScale(.large)
        Spacer()
        Button("Grant Notification Permission") {
          model.grantNotificationButtonTapped(errorHandler: errorHandler)
        }
        .buttonStyle(.borderedProminent)
        .controlSize(.large)
        Spacer()
          .frame(maxHeight: 20)
        Text(
          "After tapping the above button, tap 'Allow' when prompted."
        )
        .font(.caption)
        .multilineTextAlignment(.center)
        Spacer()
      })
  }
}
