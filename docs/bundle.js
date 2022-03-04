(()=>{"use strict";var e,t,n,r,o,a,c={367:(e,t,n)=>{var r=n(871);let o=document.getElementsByClassName("tabs");Array.from(o).forEach((e=>{e.addEventListener("click",(e=>{e.preventDefault(),(0,r.A)(e.target.id)}))})),Promise.resolve().then(n.bind(n,235)).then((e=>{let t=document.getElementById("reference-input"),n=document.getElementById("eastings-output"),r=document.getElementById("northings-output");document.getElementById("submit-reference").addEventListener("click",(function(o){o.preventDefault();let a=e.reference_to_coordinates(t.value);n.value=a.eastings,r.value=a.northings}));let o=document.getElementById("eastings-input"),a=document.getElementById("northings-input"),c=document.getElementById("reference-output");document.getElementById("submit-coordinates").addEventListener("click",(function(t){t.preventDefault();let n=e.coordinates_to_reference(o.value,a.value);c.value=n}))})).catch((e=>console.error(e)))},871:(e,t,n)=>{n.d(t,{A:()=>r});const r=e=>{let t=document.getElementsByClassName("tab");Array.from(t).forEach((t=>{t.id==e?(t.classList.add("tab-selected"),t.classList.remove("tab-deselected")):(t.classList.remove("tab-selected"),t.classList.add("tab-deselected"))}));let n=document.getElementsByClassName("tab-content");Array.from(n).forEach((t=>{t.id==e?t.classList.remove("inactive"):t.classList.add("inactive")}))}},235:(e,t,n)=>{n.a(e,(async(e,r)=>{try{n.r(t),n.d(t,{__wbindgen_json_parse:()=>o.t$,coordinates_to_reference:()=>o.s5,reference_to_coordinates:()=>o.ei});var o=n(838),a=e([o]);o=(a.then?(await a)():a)[0],r()}catch(e){r(e)}}))},838:(e,t,n)=>{n.a(e,(async(r,o)=>{try{n.d(t,{ei:()=>w,s5:()=>S,t$:()=>B});var a=n(530);e=n.hmd(e);var c=r([a]);a=(c.then?(await c)():c)[0];let i=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});i.decode();let s=null;function l(){return null!==s&&s.buffer===a.memory.buffer||(s=new Uint8Array(a.memory.buffer)),s}function u(e,t){return i.decode(l().subarray(e,e+t))}const d=new Array(32).fill(void 0);d.push(void 0,null,!0,!1);let f=d.length;function m(e){f===d.length&&d.push(d.length+1);const t=f;return f=d[t],d[t]=e,t}let b=0,y=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const _="function"==typeof y.encodeInto?function(e,t){return y.encodeInto(e,t)}:function(e,t){const n=y.encode(e);return t.set(n),{read:e.length,written:n.length}};function p(e,t,n){if(void 0===n){const n=y.encode(e),r=t(n.length);return l().subarray(r,r+n.length).set(n),b=n.length,r}let r=e.length,o=t(r);const a=l();let c=0;for(;c<r;c++){const t=e.charCodeAt(c);if(t>127)break;a[o+c]=t}if(c!==r){0!==c&&(e=e.slice(c)),o=n(o,r,r=c+3*e.length);const t=l().subarray(o+c,o+r);c+=_(e,t).written}return b=c,o}function h(e){return d[e]}function g(e){e<36||(d[e]=f,f=e)}function v(e){const t=h(e);return g(e),t}function w(e){var t=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc),n=b;return v(a.reference_to_coordinates(t,n))}let E=null;function x(){return null!==E&&E.buffer===a.memory.buffer||(E=new Int32Array(a.memory.buffer)),E}function S(e,t){try{const o=a.__wbindgen_add_to_stack_pointer(-16);a.coordinates_to_reference(o,e,t);var n=x()[o/4+0],r=x()[o/4+1];return u(n,r)}finally{a.__wbindgen_add_to_stack_pointer(16),a.__wbindgen_free(n,r)}}function B(e,t){return m(JSON.parse(u(e,t)))}o()}catch(e){o(e)}}))},530:(e,t,n)=>{n.a(e,(async(r,o)=>{try{var a,c=r([a=n(838)]),[a]=c.then?(await c)():c;await n.v(t,e.id,"3588b93ae873b876324b",{"./index_bg.js":{__wbindgen_json_parse:a.t$}}),o()}catch(e){o(e)}}),1)}},i={};function s(e){var t=i[e];if(void 0!==t)return t.exports;var n=i[e]={id:e,loaded:!1,exports:{}};return c[e](n,n.exports,s),n.loaded=!0,n.exports}e="function"==typeof Symbol?Symbol("webpack then"):"__webpack_then__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",n="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&(e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},o=e=>!--e.r&&e(),a=(e,t)=>e?e.push(t):o(t),s.a=(c,i,s)=>{var l,u,d,f=s&&[],m=c.exports,b=!0,y=!1,_=(t,n,r)=>{y||(y=!0,n.r+=t.length,t.map(((t,o)=>t[e](n,r))),y=!1)},p=new Promise(((e,t)=>{d=t,u=()=>(e(m),r(f),f=0)}));p[t]=m,p[e]=(e,t)=>{if(b)return o(e);l&&_(l,e,t),a(f,e),p.catch(t)},c.exports=p,i((c=>{var i;l=(c=>c.map((c=>{if(null!==c&&"object"==typeof c){if(c[e])return c;if(c.then){var i=[];c.then((e=>{s[t]=e,r(i),i=0}),(e=>{s[n]=e,r(i),i=0}));var s={};return s[e]=(e,t)=>(a(i,e),c.catch(t)),s}}var l={};return l[e]=e=>o(e),l[t]=c,l})))(c);var s=()=>l.map((e=>{if(e[n])throw e[n];return e[t]})),u=new Promise(((e,t)=>{(i=()=>e(s)).r=0,_(l,i,t)}));return i.r?u:s()}),(e=>(e&&d(p[n]=e),u()))),b=!1},s.d=(e,t)=>{for(var n in t)s.o(t,n)&&!s.o(e,n)&&Object.defineProperty(e,n,{enumerable:!0,get:t[n]})},s.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),s.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),s.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},s.v=(e,t,n,r)=>{var o=fetch(s.p+""+n+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,r).then((t=>Object.assign(e,t.instance.exports))):o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((t=>Object.assign(e,t.instance.exports)))},s.p="",s(367),s(871),s(235)})();