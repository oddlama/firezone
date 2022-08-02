"use strict";(self.webpackChunknew_docs=self.webpackChunknew_docs||[]).push([[1346],{3905:(e,r,t)=>{t.d(r,{Zo:()=>c,kt:()=>f});var n=t(7294);function s(e,r,t){return r in e?Object.defineProperty(e,r,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[r]=t,e}function o(e,r){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);r&&(n=n.filter((function(r){return Object.getOwnPropertyDescriptor(e,r).enumerable}))),t.push.apply(t,n)}return t}function i(e){for(var r=1;r<arguments.length;r++){var t=null!=arguments[r]?arguments[r]:{};r%2?o(Object(t),!0).forEach((function(r){s(e,r,t[r])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):o(Object(t)).forEach((function(r){Object.defineProperty(e,r,Object.getOwnPropertyDescriptor(t,r))}))}return e}function l(e,r){if(null==e)return{};var t,n,s=function(e,r){if(null==e)return{};var t,n,s={},o=Object.keys(e);for(n=0;n<o.length;n++)t=o[n],r.indexOf(t)>=0||(s[t]=e[t]);return s}(e,r);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(n=0;n<o.length;n++)t=o[n],r.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(s[t]=e[t])}return s}var a=n.createContext({}),u=function(e){var r=n.useContext(a),t=r;return e&&(t="function"==typeof e?e(r):i(i({},r),e)),t},c=function(e){var r=u(e.components);return n.createElement(a.Provider,{value:r},e.children)},p={inlineCode:"code",wrapper:function(e){var r=e.children;return n.createElement(n.Fragment,{},r)}},d=n.forwardRef((function(e,r){var t=e.components,s=e.mdxType,o=e.originalType,a=e.parentName,c=l(e,["components","mdxType","originalType","parentName"]),d=u(t),f=s,g=d["".concat(a,".").concat(f)]||d[f]||p[f]||o;return t?n.createElement(g,i(i({ref:r},c),{},{components:t})):n.createElement(g,i({ref:r},c))}));function f(e,r){var t=arguments,s=r&&r.mdxType;if("string"==typeof e||s){var o=t.length,i=new Array(o);i[0]=d;var l={};for(var a in r)hasOwnProperty.call(r,a)&&(l[a]=r[a]);l.originalType=e,l.mdxType="string"==typeof e?e:s,i[1]=l;for(var u=2;u<o;u++)i[u]=t[u];return n.createElement.apply(null,i)}return n.createElement.apply(null,t)}d.displayName="MDXCreateElement"},608:(e,r,t)=>{t.r(r),t.d(r,{assets:()=>a,contentTitle:()=>i,default:()=>p,frontMatter:()=>o,metadata:()=>l,toc:()=>u});var n=t(7462),s=(t(7294),t(3905));const o={title:"Egress Rules",sidebar_position:3},i=void 0,l={unversionedId:"user-guides/egress-rules",id:"user-guides/egress-rules",title:"Egress Rules",description:"Firezone supports egress filtering controls to explicitly DROP or ACCEPT packets",source:"@site/docs/user-guides/egress-rules.md",sourceDirName:"user-guides",slug:"/user-guides/egress-rules",permalink:"/user-guides/egress-rules",draft:!1,editUrl:"https://github.com/firezone/firezone/docs/user-guides/egress-rules.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{title:"Egress Rules",sidebar_position:3},sidebar:"tutorialSidebar",previous:{title:"Add Devices",permalink:"/user-guides/add-devices"},next:{title:"Client Instructions",permalink:"/user-guides/client-instructions"}},a={},u=[],c={toc:u};function p(e){let{components:r,...t}=e;return(0,s.kt)("wrapper",(0,n.Z)({},c,t,{components:r,mdxType:"MDXLayout"}),(0,s.kt)("p",null,"Firezone supports egress filtering controls to explicitly DROP or ACCEPT packets\nvia the kernel's netfilter system. By default, all traffic is allowed."),(0,s.kt)("p",null,"The Allowlist and Denylist support both IPv4 and IPv6 CIDRs and IP addresses.\nWhen adding a rule, you may optionally scope it to a user which applies the\nrule to all their devices."),(0,s.kt)("p",null,(0,s.kt)("img",{parentName:"p",src:"https://user-images.githubusercontent.com/69542737/177398544-89df7f6b-9296-4c11-ba6a-e95c72f853b7.png",alt:"firewall_rules"})))}p.isMDXComponent=!0}}]);