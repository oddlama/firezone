"use strict";(self.webpackChunknew_docs=self.webpackChunknew_docs||[]).push([[7085],{3905:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>m});var r=n(7294);function i(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){i(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,i=function(e,t){if(null==e)return{};var n,r,i={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(i[n]=e[n]);return i}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(i[n]=e[n])}return i}var s=r.createContext({}),d=function(e){var t=r.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},p=function(e){var t=d(e.components);return r.createElement(s.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},c=r.forwardRef((function(e,t){var n=e.components,i=e.mdxType,a=e.originalType,s=e.parentName,p=l(e,["components","mdxType","originalType","parentName"]),c=d(n),m=i,y=c["".concat(s,".").concat(m)]||c[m]||u[m]||a;return n?r.createElement(y,o(o({ref:t},p),{},{components:n})):r.createElement(y,o({ref:t},p))}));function m(e,t){var n=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var a=n.length,o=new Array(a);o[0]=c;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l.mdxType="string"==typeof e?e:i,o[1]=l;for(var d=2;d<a;d++)o[d]=n[d];return r.createElement.apply(null,o)}return r.createElement.apply(null,n)}c.displayName="MDXCreateElement"},6961:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>o,default:()=>u,frontMatter:()=>a,metadata:()=>l,toc:()=>d});var r=n(7462),i=(n(7294),n(3905));const a={title:"Security Considerations",sidebar_position:6},o=void 0,l={unversionedId:"administer/security-considerations",id:"administer/security-considerations",title:"Security Considerations",description:"Disclaimer: Firezone is still beta software. The codebase has not yet",source:"@site/docs/administer/security-considerations.md",sourceDirName:"administer",slug:"/administer/security-considerations",permalink:"/administer/security-considerations",draft:!1,editUrl:"https://github.com/firezone/firezone/docs/administer/security-considerations.md",tags:[],version:"current",sidebarPosition:6,frontMatter:{title:"Security Considerations",sidebar_position:6},sidebar:"tutorialSidebar",previous:{title:"Troubleshoot",permalink:"/administer/troubleshoot"},next:{title:"Running SQL Queries",permalink:"/administer/running-sql-queries"}},s={},d=[{value:"List of services and ports",id:"list-of-services-and-ports",level:2},{value:"Production deployments",id:"production-deployments",level:2},{value:"Reporting Security Issues",id:"reporting-security-issues",level:2}],p={toc:d};function u(e){let{components:t,...n}=e;return(0,i.kt)("wrapper",(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("p",null,(0,i.kt)("strong",{parentName:"p"},"Disclaimer"),": Firezone is still beta software. The codebase has not yet\nreceived a formal security audit. For highly sensitive and mission-critical\nproduction deployments, we recommend limiting access to the web interface, as\ndetailed ",(0,i.kt)("a",{parentName:"p",href:"#production-deployments"},"below"),"."),(0,i.kt)("h2",{id:"list-of-services-and-ports"},"List of services and ports"),(0,i.kt)("p",null,"Shown below is a table of ports used by Firezone services."),(0,i.kt)("table",null,(0,i.kt)("thead",{parentName:"table"},(0,i.kt)("tr",{parentName:"thead"},(0,i.kt)("th",{parentName:"tr",align:null},"Service"),(0,i.kt)("th",{parentName:"tr",align:null},"Default port"),(0,i.kt)("th",{parentName:"tr",align:null},"Listen address"),(0,i.kt)("th",{parentName:"tr",align:null},"Description"))),(0,i.kt)("tbody",{parentName:"table"},(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},"Nginx"),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"80")," ",(0,i.kt)("inlineCode",{parentName:"td"},"443")),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"all")),(0,i.kt)("td",{parentName:"tr",align:null},"Public HTTP(S) port for administering Firezone and facilitating authentication.")),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},"WireGuard"),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"51820")),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"all")),(0,i.kt)("td",{parentName:"tr",align:null},"Public WireGuard port used for VPN sessions.")),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},"Postgresql"),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"15432")),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"127.0.0.1")),(0,i.kt)("td",{parentName:"tr",align:null},"Local-only port used for bundled Postgresql server.")),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},"Phoenix"),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"13000")),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"127.0.0.1")),(0,i.kt)("td",{parentName:"tr",align:null},"Local-only port used by upstream elixir app server.")))),(0,i.kt)("h2",{id:"production-deployments"},"Production deployments"),(0,i.kt)("p",null,"For production and public-facing deployments where a single administrator\nwill be responsible for generating and distributing device configurations to\nend users, we advise you to consider limiting access to Firezone's publicly\nexposed web UI (by default ports ",(0,i.kt)("inlineCode",{parentName:"p"},"443/tcp")," and ",(0,i.kt)("inlineCode",{parentName:"p"},"80/tcp"),")\nand instead use the WireGuard tunnel itself to manage Firezone."),(0,i.kt)("p",null,"For example, assuming an administrator has generated a device configuration and\nestablished a tunnel with local WireGuard address ",(0,i.kt)("inlineCode",{parentName:"p"},"10.3.2.2"),", the following ",(0,i.kt)("inlineCode",{parentName:"p"},"ufw"),"\nconfiguration would allow the administrator the ability to reach the Firezone web\nUI on the default ",(0,i.kt)("inlineCode",{parentName:"p"},"10.3.2.1")," tunnel address for the server's ",(0,i.kt)("inlineCode",{parentName:"p"},"wg-firezone")," interface:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-text"},"root@demo:~# ufw status verbose\nStatus: active\nLogging: on (low)\nDefault: deny (incoming), allow (outgoing), allow (routed)\nNew profiles: skip\n\nTo                         Action      From\n--                         ------      ----\n22/tcp                     ALLOW IN    Anywhere\n51820/udp                  ALLOW IN    Anywhere\nAnywhere                   ALLOW IN    10.3.2.2\n22/tcp (v6)                ALLOW IN    Anywhere (v6)\n51820/udp (v6)             ALLOW IN    Anywhere (v6)\n")),(0,i.kt)("p",null,"This would leave only ",(0,i.kt)("inlineCode",{parentName:"p"},"22/tcp")," exposed for SSH access to manage the server (optional),\nand ",(0,i.kt)("inlineCode",{parentName:"p"},"51820/udp")," exposed in order to establish WireGuard tunnels."),(0,i.kt)("admonition",{type:"note"},(0,i.kt)("p",{parentName:"admonition"},"This type of configuration has not been fully tested with SSO\nauthentication and may it to break or behave unexpectedly.")),(0,i.kt)("h2",{id:"reporting-security-issues"},"Reporting Security Issues"),(0,i.kt)("p",null,"To report any security-related bugs, see ",(0,i.kt)("a",{parentName:"p",href:"https://github.com/firezone/firezone/blob/master/SECURITY.md"},"our security bug reporting policy\n"),"."))}u.isMDXComponent=!0}}]);