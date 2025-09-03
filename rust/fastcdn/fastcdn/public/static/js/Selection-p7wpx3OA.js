import{ao as Mt,bb as me,f as $,i as x,p as N,a4 as Nt,k as Pe,l as V,bc as Kt,y as F,m as Se,j as B,g as G,r as Oe,bd as Dt,a0 as Le,an as ke,be as Te,bf as Wt,x as Ht,a7 as jt,bg as Gt,a$ as ee,bh as Vt}from"./bootstrap-znN1ihFp.js";import{u as Ut}from"./use-locale-qGUlvW_R.js";import{d as Z,z as Me,h as v,v as Zt,g as A,j as Ie,w as te,f as qt,c as L,P as Yt,E as ne,F as Jt,V as Qt}from"../jse/index-index-aJ4YWfRp.js";import{a as Xt}from"./Suffix-BnQD-wte.js";import{c as en,a as tn}from"./Follower-DbXtbDM9.js";import{N as nn}from"./Popover-QN_EeJEo.js";const M="v-hidden",on=en("[v-hidden]",{display:"none!important"}),_e=Z({name:"Overflow",props:{getCounter:Function,getTail:Function,updateCounter:Function,onUpdateCount:Function,onUpdateOverflow:Function},setup(e,{slots:n}){const t=A(null),o=A(null);function a(l){const{value:r}=t,{getCounter:p,getTail:d}=e;let c;if(p!==void 0?c=p():c=o.value,!r||!c)return;c.hasAttribute(M)&&c.removeAttribute(M);const{children:s}=r;if(l.showAllItemsBeforeCalculate)for(const w of s)w.hasAttribute(M)&&w.removeAttribute(M);const C=r.offsetWidth,m=[],g=n.tail?d==null?void 0:d():null;let y=g?g.offsetWidth:0,k=!1;const P=r.children.length-(n.tail?1:0);for(let w=0;w<P-1;++w){if(w<0)continue;const u=s[w];if(k){u.hasAttribute(M)||u.setAttribute(M,"");continue}else u.hasAttribute(M)&&u.removeAttribute(M);const b=u.offsetWidth;if(y+=b,m[w]=b,y>C){const{updateCounter:z}=e;for(let I=w;I>=0;--I){const E=P-1-I;z!==void 0?z(E):c.textContent=`${E}`;const O=c.offsetWidth;if(y-=m[I],y+O<=C||I===0){k=!0,w=I-1,g&&(w===-1?(g.style.maxWidth=`${C-O}px`,g.style.boxSizing="border-box"):g.style.maxWidth="");const{onUpdateCount:S}=e;S&&S(E);break}}}}const{onUpdateOverflow:R}=e;k?R!==void 0&&R(!0):(R!==void 0&&R(!1),c.setAttribute(M,""))}const f=Mt();return on.mount({id:"vueuc/overflow",head:!0,anchorMetaName:tn,ssr:f}),Ie(()=>a({showAllItemsBeforeCalculate:!1})),{selfRef:t,counterRef:o,sync:a}},render(){const{$slots:e}=this;return Me(()=>this.sync({showAllItemsBeforeCalculate:!1})),v("div",{class:"v-overflow",ref:"selfRef"},[Zt(e,"default"),e.counter?e.counter():v("span",{style:{display:"inline-block"},ref:"counterRef"}),e.tail?e.tail():null])}});function rn(e,n){n&&(Ie(()=>{const{value:t}=e;t&&me.registerHandler(t,n)}),te(e,(t,o)=>{o&&me.unregisterHandler(o)},{deep:!1}),qt(()=>{const{value:t}=e;t&&me.unregisterHandler(t)}))}function Ae(e){switch(typeof e){case"string":return e||void 0;case"number":return String(e);default:return}}const ln=Z({name:"Empty",render(){return v("svg",{viewBox:"0 0 28 28",fill:"none",xmlns:"http://www.w3.org/2000/svg"},v("path",{d:"M26 7.5C26 11.0899 23.0899 14 19.5 14C15.9101 14 13 11.0899 13 7.5C13 3.91015 15.9101 1 19.5 1C23.0899 1 26 3.91015 26 7.5ZM16.8536 4.14645C16.6583 3.95118 16.3417 3.95118 16.1464 4.14645C15.9512 4.34171 15.9512 4.65829 16.1464 4.85355L18.7929 7.5L16.1464 10.1464C15.9512 10.3417 15.9512 10.6583 16.1464 10.8536C16.3417 11.0488 16.6583 11.0488 16.8536 10.8536L19.5 8.20711L22.1464 10.8536C22.3417 11.0488 22.6583 11.0488 22.8536 10.8536C23.0488 10.6583 23.0488 10.3417 22.8536 10.1464L20.2071 7.5L22.8536 4.85355C23.0488 4.65829 23.0488 4.34171 22.8536 4.14645C22.6583 3.95118 22.3417 3.95118 22.1464 4.14645L19.5 6.79289L16.8536 4.14645Z",fill:"currentColor"}),v("path",{d:"M25 22.75V12.5991C24.5572 13.0765 24.053 13.4961 23.5 13.8454V16H17.5L17.3982 16.0068C17.0322 16.0565 16.75 16.3703 16.75 16.75C16.75 18.2688 15.5188 19.5 14 19.5C12.4812 19.5 11.25 18.2688 11.25 16.75L11.2432 16.6482C11.1935 16.2822 10.8797 16 10.5 16H4.5V7.25C4.5 6.2835 5.2835 5.5 6.25 5.5H12.2696C12.4146 4.97463 12.6153 4.47237 12.865 4H6.25C4.45507 4 3 5.45507 3 7.25V22.75C3 24.5449 4.45507 26 6.25 26H21.75C23.5449 26 25 24.5449 25 22.75ZM4.5 22.75V17.5H9.81597L9.85751 17.7041C10.2905 19.5919 11.9808 21 14 21L14.215 20.9947C16.2095 20.8953 17.842 19.4209 18.184 17.5H23.5V22.75C23.5 23.7165 22.7165 24.5 21.75 24.5H6.25C5.2835 24.5 4.5 23.7165 4.5 22.75Z",fill:"currentColor"}))}});function Fe(e){return Array.isArray(e)?e:[e]}const ze={STOP:"STOP"};function Ne(e,n){const t=n(e);e.children!==void 0&&t!==ze.STOP&&e.children.forEach(o=>Ne(o,n))}function an(e,n={}){const{preserveGroup:t=!1}=n,o=[],a=t?l=>{l.isLeaf||(o.push(l.key),f(l.children))}:l=>{l.isLeaf||(l.isGroup||o.push(l.key),f(l.children))};function f(l){l.forEach(a)}return f(e),o}function sn(e,n){const{isLeaf:t}=e;return t!==void 0?t:!n(e)}function cn(e){return e.children}function dn(e){return e.key}function un(){return!1}function hn(e,n){const{isLeaf:t}=e;return!(t===!1&&!Array.isArray(n(e)))}function fn(e){return e.disabled===!0}function vn(e,n){return e.isLeaf===!1&&!Array.isArray(n(e))}function Ce(e){var n;return e==null?[]:Array.isArray(e)?e:(n=e.checkedKeys)!==null&&n!==void 0?n:[]}function ye(e){var n;return e==null||Array.isArray(e)?[]:(n=e.indeterminateKeys)!==null&&n!==void 0?n:[]}function bn(e,n){const t=new Set(e);return n.forEach(o=>{t.has(o)||t.add(o)}),Array.from(t)}function gn(e,n){const t=new Set(e);return n.forEach(o=>{t.has(o)&&t.delete(o)}),Array.from(t)}function pn(e){return(e==null?void 0:e.type)==="group"}function Hn(e){const n=new Map;return e.forEach((t,o)=>{n.set(t.key,o)}),t=>{var o;return(o=n.get(t))!==null&&o!==void 0?o:null}}class mn extends Error{constructor(){super(),this.message="SubtreeNotLoadedError: checking a subtree whose required nodes are not fully loaded."}}function Cn(e,n,t,o){return oe(n.concat(e),t,o,!1)}function yn(e,n){const t=new Set;return e.forEach(o=>{const a=n.treeNodeMap.get(o);if(a!==void 0){let f=a.parent;for(;f!==null&&!(f.disabled||t.has(f.key));)t.add(f.key),f=f.parent}}),t}function wn(e,n,t,o){const a=oe(n,t,o,!1),f=oe(e,t,o,!0),l=yn(e,t),r=[];return a.forEach(p=>{(f.has(p)||l.has(p))&&r.push(p)}),r.forEach(p=>a.delete(p)),a}function we(e,n){const{checkedKeys:t,keysToCheck:o,keysToUncheck:a,indeterminateKeys:f,cascade:l,leafOnly:r,checkStrategy:p,allowNotLoaded:d}=e;if(!l)return o!==void 0?{checkedKeys:bn(t,o),indeterminateKeys:Array.from(f)}:a!==void 0?{checkedKeys:gn(t,a),indeterminateKeys:Array.from(f)}:{checkedKeys:Array.from(t),indeterminateKeys:Array.from(f)};const{levelTreeNodeMap:c}=n;let s;a!==void 0?s=wn(a,t,n,d):o!==void 0?s=Cn(o,t,n,d):s=oe(t,n,d,!1);const C=p==="parent",m=p==="child"||r,g=s,y=new Set,k=Math.max.apply(null,Array.from(c.keys()));for(let P=k;P>=0;P-=1){const R=P===0,w=c.get(P);for(const u of w){if(u.isLeaf)continue;const{key:b,shallowLoaded:z}=u;if(m&&z&&u.children.forEach(S=>{!S.disabled&&!S.isLeaf&&S.shallowLoaded&&g.has(S.key)&&g.delete(S.key)}),u.disabled||!z)continue;let I=!0,E=!1,O=!0;for(const S of u.children){const K=S.key;if(!S.disabled){if(O&&(O=!1),g.has(K))E=!0;else if(y.has(K)){E=!0,I=!1;break}else if(I=!1,E)break}}I&&!O?(C&&u.children.forEach(S=>{!S.disabled&&g.has(S.key)&&g.delete(S.key)}),g.add(b)):E&&y.add(b),R&&m&&g.has(b)&&g.delete(b)}}return{checkedKeys:Array.from(g),indeterminateKeys:Array.from(y)}}function oe(e,n,t,o){const{treeNodeMap:a,getChildren:f}=n,l=new Set,r=new Set(e);return e.forEach(p=>{const d=a.get(p);d!==void 0&&Ne(d,c=>{if(c.disabled)return ze.STOP;const{key:s}=c;if(!l.has(s)&&(l.add(s),r.add(s),vn(c.rawNode,f))){if(o)return ze.STOP;if(!t)throw new mn}})}),r}function xn(e,{includeGroup:n=!1,includeSelf:t=!0},o){var a;const f=o.treeNodeMap;let l=e==null?null:(a=f.get(e))!==null&&a!==void 0?a:null;const r={keyPath:[],treeNodePath:[],treeNode:l};if(l!=null&&l.ignored)return r.treeNode=null,r;for(;l;)!l.ignored&&(n||!l.isGroup)&&r.treeNodePath.push(l),l=l.parent;return r.treeNodePath.reverse(),t||r.treeNodePath.pop(),r.keyPath=r.treeNodePath.map(p=>p.key),r}function kn(e){if(e.length===0)return null;const n=e[0];return n.isGroup||n.ignored||n.disabled?n.getNext():n}function zn(e,n){const t=e.siblings,o=t.length,{index:a}=e;return n?t[(a+1)%o]:a===t.length-1?null:t[a+1]}function Be(e,n,{loop:t=!1,includeDisabled:o=!1}={}){const a=n==="prev"?Pn:zn,f={reverse:n==="prev"};let l=!1,r=null;function p(d){if(d!==null){if(d===e){if(!l)l=!0;else if(!e.disabled&&!e.isGroup){r=e;return}}else if((!d.disabled||o)&&!d.ignored&&!d.isGroup){r=d;return}if(d.isGroup){const c=$e(d,f);c!==null?r=c:p(a(d,t))}else{const c=a(d,!1);if(c!==null)p(c);else{const s=Sn(d);s!=null&&s.isGroup?p(a(s,t)):t&&p(a(d,!0))}}}}return p(e),r}function Pn(e,n){const t=e.siblings,o=t.length,{index:a}=e;return n?t[(a-1+o)%o]:a===0?null:t[a-1]}function Sn(e){return e.parent}function $e(e,n={}){const{reverse:t=!1}=n,{children:o}=e;if(o){const{length:a}=o,f=t?a-1:0,l=t?-1:a,r=t?-1:1;for(let p=f;p!==l;p+=r){const d=o[p];if(!d.disabled&&!d.ignored)if(d.isGroup){const c=$e(d,n);if(c!==null)return c}else return d}}return null}const In={getChild(){return this.ignored?null:$e(this)},getParent(){const{parent:e}=this;return e!=null&&e.isGroup?e.getParent():e},getNext(e={}){return Be(this,"next",e)},getPrev(e={}){return Be(this,"prev",e)}};function $n(e,n){const t=n?new Set(n):void 0,o=[];function a(f){f.forEach(l=>{o.push(l),!(l.isLeaf||!l.children||l.ignored)&&(l.isGroup||t===void 0||t.has(l.key))&&a(l.children)})}return a(e),o}function Rn(e,n){const t=e.key;for(;n;){if(n.key===t)return!0;n=n.parent}return!1}function Ke(e,n,t,o,a,f=null,l=0){const r=[];return e.forEach((p,d)=>{var c;const s=Object.create(o);if(s.rawNode=p,s.siblings=r,s.level=l,s.index=d,s.isFirstChild=d===0,s.isLastChild=d+1===e.length,s.parent=f,!s.ignored){const C=a(p);Array.isArray(C)&&(s.children=Ke(C,n,t,o,a,s,l+1))}r.push(s),n.set(s.key,s),t.has(l)||t.set(l,[]),(c=t.get(l))===null||c===void 0||c.push(s)}),r}function jn(e,n={}){var t;const o=new Map,a=new Map,{getDisabled:f=fn,getIgnored:l=un,getIsGroup:r=pn,getKey:p=dn}=n,d=(t=n.getChildren)!==null&&t!==void 0?t:cn,c=n.ignoreEmptyChildren?u=>{const b=d(u);return Array.isArray(b)?b.length?b:null:b}:d,s=Object.assign({get key(){return p(this.rawNode)},get disabled(){return f(this.rawNode)},get isGroup(){return r(this.rawNode)},get isLeaf(){return sn(this.rawNode,c)},get shallowLoaded(){return hn(this.rawNode,c)},get ignored(){return l(this.rawNode)},contains(u){return Rn(this,u)}},In),C=Ke(e,o,a,s,c);function m(u){if(u==null)return null;const b=o.get(u);return b&&!b.isGroup&&!b.ignored?b:null}function g(u){if(u==null)return null;const b=o.get(u);return b&&!b.ignored?b:null}function y(u,b){const z=g(u);return z?z.getPrev(b):null}function k(u,b){const z=g(u);return z?z.getNext(b):null}function P(u){const b=g(u);return b?b.getParent():null}function R(u){const b=g(u);return b?b.getChild():null}const w={treeNodes:C,treeNodeMap:o,levelTreeNodeMap:a,maxLevel:Math.max(...a.keys()),getChildren:c,getFlattenedNodes(u){return $n(C,u)},getNode:m,getPrev:y,getNext:k,getParent:P,getChild:R,getFirstAvailableNode(){return kn(C)},getPath(u,b={}){return xn(u,b,w)},getCheckedKeys(u,b={}){const{cascade:z=!0,leafOnly:I=!1,checkStrategy:E="all",allowNotLoaded:O=!1}=b;return we({checkedKeys:Ce(u),indeterminateKeys:ye(u),cascade:z,leafOnly:I,checkStrategy:E,allowNotLoaded:O},w)},check(u,b,z={}){const{cascade:I=!0,leafOnly:E=!1,checkStrategy:O="all",allowNotLoaded:S=!1}=z;return we({checkedKeys:Ce(b),indeterminateKeys:ye(b),keysToCheck:u==null?[]:Fe(u),cascade:I,leafOnly:E,checkStrategy:O,allowNotLoaded:S},w)},uncheck(u,b,z={}){const{cascade:I=!0,leafOnly:E=!1,checkStrategy:O="all",allowNotLoaded:S=!1}=z;return we({checkedKeys:Ce(b),indeterminateKeys:ye(b),keysToUncheck:u==null?[]:Fe(u),cascade:I,leafOnly:E,checkStrategy:O,allowNotLoaded:S},w)},getNonLeafKeys(u={}){return an(C,u)}};return w}const En=$("empty",`
 display: flex;
 flex-direction: column;
 align-items: center;
 font-size: var(--n-font-size);
`,[x("icon",`
 width: var(--n-icon-size);
 height: var(--n-icon-size);
 font-size: var(--n-icon-size);
 line-height: var(--n-icon-size);
 color: var(--n-icon-color);
 transition:
 color .3s var(--n-bezier);
 `,[N("+",[x("description",`
 margin-top: 8px;
 `)])]),x("description",`
 transition: color .3s var(--n-bezier);
 color: var(--n-text-color);
 `),x("extra",`
 text-align: center;
 transition: color .3s var(--n-bezier);
 margin-top: 12px;
 color: var(--n-extra-text-color);
 `)]),On=Object.assign(Object.assign({},V.props),{description:String,showDescription:{type:Boolean,default:!0},showIcon:{type:Boolean,default:!0},size:{type:String,default:"medium"},renderIcon:Function}),Gn=Z({name:"Empty",props:On,slots:Object,setup(e){const{mergedClsPrefixRef:n,inlineThemeDisabled:t,mergedComponentPropsRef:o}=Pe(e),a=V("Empty","-empty",En,Kt,e,n),{localeRef:f}=Ut("Empty"),l=L(()=>{var c,s,C;return(c=e.description)!==null&&c!==void 0?c:(C=(s=o==null?void 0:o.value)===null||s===void 0?void 0:s.Empty)===null||C===void 0?void 0:C.description}),r=L(()=>{var c,s;return((s=(c=o==null?void 0:o.value)===null||c===void 0?void 0:c.Empty)===null||s===void 0?void 0:s.renderIcon)||(()=>v(ln,null))}),p=L(()=>{const{size:c}=e,{common:{cubicBezierEaseInOut:s},self:{[F("iconSize",c)]:C,[F("fontSize",c)]:m,textColor:g,iconColor:y,extraTextColor:k}}=a.value;return{"--n-icon-size":C,"--n-font-size":m,"--n-bezier":s,"--n-text-color":g,"--n-icon-color":y,"--n-extra-text-color":k}}),d=t?Se("empty",L(()=>{let c="";const{size:s}=e;return c+=s[0],c}),p,e):void 0;return{mergedClsPrefix:n,mergedRenderIcon:r,localizedDescription:L(()=>l.value||f.value.description),cssVars:t?void 0:p,themeClass:d==null?void 0:d.themeClass,onRender:d==null?void 0:d.onRender}},render(){const{$slots:e,mergedClsPrefix:n,onRender:t}=this;return t==null||t(),v("div",{class:[`${n}-empty`,this.themeClass],style:this.cssVars},this.showIcon?v("div",{class:`${n}-empty__icon`},e.icon?e.icon():v(Nt,{clsPrefix:n},{default:this.mergedRenderIcon})):null,this.showDescription?v("div",{class:`${n}-empty__description`},e.default?e.default():this.localizedDescription):null,e.extra?v("div",{class:`${n}-empty__extra`},e.extra()):null)}}),Tn={color:Object,type:{type:String,default:"default"},round:Boolean,size:{type:String,default:"medium"},closable:Boolean,disabled:{type:Boolean,default:void 0}},_n=$("tag",`
 --n-close-margin: var(--n-close-margin-top) var(--n-close-margin-right) var(--n-close-margin-bottom) var(--n-close-margin-left);
 white-space: nowrap;
 position: relative;
 box-sizing: border-box;
 cursor: default;
 display: inline-flex;
 align-items: center;
 flex-wrap: nowrap;
 padding: var(--n-padding);
 border-radius: var(--n-border-radius);
 color: var(--n-text-color);
 background-color: var(--n-color);
 transition: 
 border-color .3s var(--n-bezier),
 background-color .3s var(--n-bezier),
 color .3s var(--n-bezier),
 box-shadow .3s var(--n-bezier),
 opacity .3s var(--n-bezier);
 line-height: 1;
 height: var(--n-height);
 font-size: var(--n-font-size);
`,[B("strong",`
 font-weight: var(--n-font-weight-strong);
 `),x("border",`
 pointer-events: none;
 position: absolute;
 left: 0;
 right: 0;
 top: 0;
 bottom: 0;
 border-radius: inherit;
 border: var(--n-border);
 transition: border-color .3s var(--n-bezier);
 `),x("icon",`
 display: flex;
 margin: 0 4px 0 0;
 color: var(--n-text-color);
 transition: color .3s var(--n-bezier);
 font-size: var(--n-avatar-size-override);
 `),x("avatar",`
 display: flex;
 margin: 0 6px 0 0;
 `),x("close",`
 margin: var(--n-close-margin);
 transition:
 background-color .3s var(--n-bezier),
 color .3s var(--n-bezier);
 `),B("round",`
 padding: 0 calc(var(--n-height) / 3);
 border-radius: calc(var(--n-height) / 2);
 `,[x("icon",`
 margin: 0 4px 0 calc((var(--n-height) - 8px) / -2);
 `),x("avatar",`
 margin: 0 6px 0 calc((var(--n-height) - 8px) / -2);
 `),B("closable",`
 padding: 0 calc(var(--n-height) / 4) 0 calc(var(--n-height) / 3);
 `)]),B("icon, avatar",[B("round",`
 padding: 0 calc(var(--n-height) / 3) 0 calc(var(--n-height) / 2);
 `)]),B("disabled",`
 cursor: not-allowed !important;
 opacity: var(--n-opacity-disabled);
 `),B("checkable",`
 cursor: pointer;
 box-shadow: none;
 color: var(--n-text-color-checkable);
 background-color: var(--n-color-checkable);
 `,[G("disabled",[N("&:hover","background-color: var(--n-color-hover-checkable);",[G("checked","color: var(--n-text-color-hover-checkable);")]),N("&:active","background-color: var(--n-color-pressed-checkable);",[G("checked","color: var(--n-text-color-pressed-checkable);")])]),B("checked",`
 color: var(--n-text-color-checked);
 background-color: var(--n-color-checked);
 `,[G("disabled",[N("&:hover","background-color: var(--n-color-checked-hover);"),N("&:active","background-color: var(--n-color-checked-pressed);")])])])]),An=Object.assign(Object.assign(Object.assign({},V.props),Tn),{bordered:{type:Boolean,default:void 0},checked:Boolean,checkable:Boolean,strong:Boolean,triggerClickOnClose:Boolean,onClose:[Array,Function],onMouseenter:Function,onMouseleave:Function,"onUpdate:checked":Function,onUpdateChecked:Function,internalCloseFocusable:{type:Boolean,default:!0},internalCloseIsButtonTag:{type:Boolean,default:!0},onCheckedChange:Function}),Fn=jt("n-tag"),xe=Z({name:"Tag",props:An,slots:Object,setup(e){const n=A(null),{mergedBorderedRef:t,mergedClsPrefixRef:o,inlineThemeDisabled:a,mergedRtlRef:f}=Pe(e),l=V("Tag","-tag",_n,Wt,e,o);Yt(Fn,{roundRef:ne(e,"round")});function r(){if(!e.disabled&&e.checkable){const{checked:m,onCheckedChange:g,onUpdateChecked:y,"onUpdate:checked":k}=e;y&&y(!m),k&&k(!m),g&&g(!m)}}function p(m){if(e.triggerClickOnClose||m.stopPropagation(),!e.disabled){const{onClose:g}=e;g&&Ht(g,m)}}const d={setTextContent(m){const{value:g}=n;g&&(g.textContent=m)}},c=Le("Tag",f,o),s=L(()=>{const{type:m,size:g,color:{color:y,textColor:k}={}}=e,{common:{cubicBezierEaseInOut:P},self:{padding:R,closeMargin:w,borderRadius:u,opacityDisabled:b,textColorCheckable:z,textColorHoverCheckable:I,textColorPressedCheckable:E,textColorChecked:O,colorCheckable:S,colorHoverCheckable:K,colorPressedCheckable:W,colorChecked:_,colorCheckedHover:re,colorCheckedPressed:ie,closeBorderRadius:le,fontWeightStrong:ae,[F("colorBordered",m)]:se,[F("closeSize",g)]:ce,[F("closeIconSize",g)]:q,[F("fontSize",g)]:D,[F("height",g)]:Y,[F("color",m)]:U,[F("textColor",m)]:de,[F("border",m)]:ue,[F("closeIconColor",m)]:J,[F("closeIconColorHover",m)]:he,[F("closeIconColorPressed",m)]:fe,[F("closeColorHover",m)]:ve,[F("closeColorPressed",m)]:be}}=l.value,H=ke(w);return{"--n-font-weight-strong":ae,"--n-avatar-size-override":`calc(${Y} - 8px)`,"--n-bezier":P,"--n-border-radius":u,"--n-border":ue,"--n-close-icon-size":q,"--n-close-color-pressed":be,"--n-close-color-hover":ve,"--n-close-border-radius":le,"--n-close-icon-color":J,"--n-close-icon-color-hover":he,"--n-close-icon-color-pressed":fe,"--n-close-icon-color-disabled":J,"--n-close-margin-top":H.top,"--n-close-margin-right":H.right,"--n-close-margin-bottom":H.bottom,"--n-close-margin-left":H.left,"--n-close-size":ce,"--n-color":y||(t.value?se:U),"--n-color-checkable":S,"--n-color-checked":_,"--n-color-checked-hover":re,"--n-color-checked-pressed":ie,"--n-color-hover-checkable":K,"--n-color-pressed-checkable":W,"--n-font-size":D,"--n-height":Y,"--n-opacity-disabled":b,"--n-padding":R,"--n-text-color":k||de,"--n-text-color-checkable":z,"--n-text-color-checked":O,"--n-text-color-hover-checkable":I,"--n-text-color-pressed-checkable":E}}),C=a?Se("tag",L(()=>{let m="";const{type:g,size:y,color:{color:k,textColor:P}={}}=e;return m+=g[0],m+=y[0],k&&(m+=`a${Te(k)}`),P&&(m+=`b${Te(P)}`),t.value&&(m+="c"),m}),s,e):void 0;return Object.assign(Object.assign({},d),{rtlEnabled:c,mergedClsPrefix:o,contentRef:n,mergedBordered:t,handleClick:r,handleCloseClick:p,cssVars:a?void 0:s,themeClass:C==null?void 0:C.themeClass,onRender:C==null?void 0:C.onRender})},render(){var e,n;const{mergedClsPrefix:t,rtlEnabled:o,closable:a,color:{borderColor:f}={},round:l,onRender:r,$slots:p}=this;r==null||r();const d=Oe(p.avatar,s=>s&&v("div",{class:`${t}-tag__avatar`},s)),c=Oe(p.icon,s=>s&&v("div",{class:`${t}-tag__icon`},s));return v("div",{class:[`${t}-tag`,this.themeClass,{[`${t}-tag--rtl`]:o,[`${t}-tag--strong`]:this.strong,[`${t}-tag--disabled`]:this.disabled,[`${t}-tag--checkable`]:this.checkable,[`${t}-tag--checked`]:this.checkable&&this.checked,[`${t}-tag--round`]:l,[`${t}-tag--avatar`]:d,[`${t}-tag--icon`]:c,[`${t}-tag--closable`]:a}],style:this.cssVars,onClick:this.handleClick,onMouseenter:this.onMouseenter,onMouseleave:this.onMouseleave},c||d,v("span",{class:`${t}-tag__content`,ref:"contentRef"},(n=(e=this.$slots).default)===null||n===void 0?void 0:n.call(e)),!this.checkable&&a?v(Dt,{clsPrefix:t,class:`${t}-tag__close`,disabled:this.disabled,onClick:this.handleCloseClick,focusable:this.internalCloseFocusable,round:l,isButtonTag:this.internalCloseIsButtonTag,absolute:!0}):null,!this.checkable&&this.mergedBordered?v("div",{class:`${t}-tag__border`,style:{borderColor:f}}):null)}}),Bn=N([$("base-selection",`
 --n-padding-single: var(--n-padding-single-top) var(--n-padding-single-right) var(--n-padding-single-bottom) var(--n-padding-single-left);
 --n-padding-multiple: var(--n-padding-multiple-top) var(--n-padding-multiple-right) var(--n-padding-multiple-bottom) var(--n-padding-multiple-left);
 position: relative;
 z-index: auto;
 box-shadow: none;
 width: 100%;
 max-width: 100%;
 display: inline-block;
 vertical-align: bottom;
 border-radius: var(--n-border-radius);
 min-height: var(--n-height);
 line-height: 1.5;
 font-size: var(--n-font-size);
 `,[$("base-loading",`
 color: var(--n-loading-color);
 `),$("base-selection-tags","min-height: var(--n-height);"),x("border, state-border",`
 position: absolute;
 left: 0;
 right: 0;
 top: 0;
 bottom: 0;
 pointer-events: none;
 border: var(--n-border);
 border-radius: inherit;
 transition:
 box-shadow .3s var(--n-bezier),
 border-color .3s var(--n-bezier);
 `),x("state-border",`
 z-index: 1;
 border-color: #0000;
 `),$("base-suffix",`
 cursor: pointer;
 position: absolute;
 top: 50%;
 transform: translateY(-50%);
 right: 10px;
 `,[x("arrow",`
 font-size: var(--n-arrow-size);
 color: var(--n-arrow-color);
 transition: color .3s var(--n-bezier);
 `)]),$("base-selection-overlay",`
 display: flex;
 align-items: center;
 white-space: nowrap;
 pointer-events: none;
 position: absolute;
 top: 0;
 right: 0;
 bottom: 0;
 left: 0;
 padding: var(--n-padding-single);
 transition: color .3s var(--n-bezier);
 `,[x("wrapper",`
 flex-basis: 0;
 flex-grow: 1;
 overflow: hidden;
 text-overflow: ellipsis;
 `)]),$("base-selection-placeholder",`
 color: var(--n-placeholder-color);
 `,[x("inner",`
 max-width: 100%;
 overflow: hidden;
 `)]),$("base-selection-tags",`
 cursor: pointer;
 outline: none;
 box-sizing: border-box;
 position: relative;
 z-index: auto;
 display: flex;
 padding: var(--n-padding-multiple);
 flex-wrap: wrap;
 align-items: center;
 width: 100%;
 vertical-align: bottom;
 background-color: var(--n-color);
 border-radius: inherit;
 transition:
 color .3s var(--n-bezier),
 box-shadow .3s var(--n-bezier),
 background-color .3s var(--n-bezier);
 `),$("base-selection-label",`
 height: var(--n-height);
 display: inline-flex;
 width: 100%;
 vertical-align: bottom;
 cursor: pointer;
 outline: none;
 z-index: auto;
 box-sizing: border-box;
 position: relative;
 transition:
 color .3s var(--n-bezier),
 box-shadow .3s var(--n-bezier),
 background-color .3s var(--n-bezier);
 border-radius: inherit;
 background-color: var(--n-color);
 align-items: center;
 `,[$("base-selection-input",`
 font-size: inherit;
 line-height: inherit;
 outline: none;
 cursor: pointer;
 box-sizing: border-box;
 border:none;
 width: 100%;
 padding: var(--n-padding-single);
 background-color: #0000;
 color: var(--n-text-color);
 transition: color .3s var(--n-bezier);
 caret-color: var(--n-caret-color);
 `,[x("content",`
 text-overflow: ellipsis;
 overflow: hidden;
 white-space: nowrap; 
 `)]),x("render-label",`
 color: var(--n-text-color);
 `)]),G("disabled",[N("&:hover",[x("state-border",`
 box-shadow: var(--n-box-shadow-hover);
 border: var(--n-border-hover);
 `)]),B("focus",[x("state-border",`
 box-shadow: var(--n-box-shadow-focus);
 border: var(--n-border-focus);
 `)]),B("active",[x("state-border",`
 box-shadow: var(--n-box-shadow-active);
 border: var(--n-border-active);
 `),$("base-selection-label","background-color: var(--n-color-active);"),$("base-selection-tags","background-color: var(--n-color-active);")])]),B("disabled","cursor: not-allowed;",[x("arrow",`
 color: var(--n-arrow-color-disabled);
 `),$("base-selection-label",`
 cursor: not-allowed;
 background-color: var(--n-color-disabled);
 `,[$("base-selection-input",`
 cursor: not-allowed;
 color: var(--n-text-color-disabled);
 `),x("render-label",`
 color: var(--n-text-color-disabled);
 `)]),$("base-selection-tags",`
 cursor: not-allowed;
 background-color: var(--n-color-disabled);
 `),$("base-selection-placeholder",`
 cursor: not-allowed;
 color: var(--n-placeholder-color-disabled);
 `)]),$("base-selection-input-tag",`
 height: calc(var(--n-height) - 6px);
 line-height: calc(var(--n-height) - 6px);
 outline: none;
 display: none;
 position: relative;
 margin-bottom: 3px;
 max-width: 100%;
 vertical-align: bottom;
 `,[x("input",`
 font-size: inherit;
 font-family: inherit;
 min-width: 1px;
 padding: 0;
 background-color: #0000;
 outline: none;
 border: none;
 max-width: 100%;
 overflow: hidden;
 width: 1em;
 line-height: inherit;
 cursor: pointer;
 color: var(--n-text-color);
 caret-color: var(--n-caret-color);
 `),x("mirror",`
 position: absolute;
 left: 0;
 top: 0;
 white-space: pre;
 visibility: hidden;
 user-select: none;
 -webkit-user-select: none;
 opacity: 0;
 `)]),["warning","error"].map(e=>B(`${e}-status`,[x("state-border",`border: var(--n-border-${e});`),G("disabled",[N("&:hover",[x("state-border",`
 box-shadow: var(--n-box-shadow-hover-${e});
 border: var(--n-border-hover-${e});
 `)]),B("active",[x("state-border",`
 box-shadow: var(--n-box-shadow-active-${e});
 border: var(--n-border-active-${e});
 `),$("base-selection-label",`background-color: var(--n-color-active-${e});`),$("base-selection-tags",`background-color: var(--n-color-active-${e});`)]),B("focus",[x("state-border",`
 box-shadow: var(--n-box-shadow-focus-${e});
 border: var(--n-border-focus-${e});
 `)])])]))]),$("base-selection-popover",`
 margin-bottom: -3px;
 display: flex;
 flex-wrap: wrap;
 margin-right: -8px;
 `),$("base-selection-tag-wrapper",`
 max-width: 100%;
 display: inline-flex;
 padding: 0 7px 3px 0;
 `,[N("&:last-child","padding-right: 0;"),$("tag",`
 font-size: 14px;
 max-width: 100%;
 `,[x("content",`
 line-height: 1.25;
 text-overflow: ellipsis;
 overflow: hidden;
 `)])])]),Vn=Z({name:"InternalSelection",props:Object.assign(Object.assign({},V.props),{clsPrefix:{type:String,required:!0},bordered:{type:Boolean,default:void 0},active:Boolean,pattern:{type:String,default:""},placeholder:String,selectedOption:{type:Object,default:null},selectedOptions:{type:Array,default:null},labelField:{type:String,default:"label"},valueField:{type:String,default:"value"},multiple:Boolean,filterable:Boolean,clearable:Boolean,disabled:Boolean,size:{type:String,default:"medium"},loading:Boolean,autofocus:Boolean,showArrow:{type:Boolean,default:!0},inputProps:Object,focused:Boolean,renderTag:Function,onKeydown:Function,onClick:Function,onBlur:Function,onFocus:Function,onDeleteOption:Function,maxTagCount:[String,Number],ellipsisTagPopoverProps:Object,onClear:Function,onPatternInput:Function,onPatternFocus:Function,onPatternBlur:Function,renderLabel:Function,status:String,inlineThemeDisabled:Boolean,ignoreComposition:{type:Boolean,default:!0},onResize:Function}),setup(e){const{mergedClsPrefixRef:n,mergedRtlRef:t}=Pe(e),o=Le("InternalSelection",t,n),a=A(null),f=A(null),l=A(null),r=A(null),p=A(null),d=A(null),c=A(null),s=A(null),C=A(null),m=A(null),g=A(!1),y=A(!1),k=A(!1),P=V("InternalSelection","-internal-selection",Bn,Vt,e,ne(e,"clsPrefix")),R=L(()=>e.clearable&&!e.disabled&&(k.value||e.active)),w=L(()=>e.selectedOption?e.renderTag?e.renderTag({option:e.selectedOption,handleClose:()=>{}}):e.renderLabel?e.renderLabel(e.selectedOption,!0):ee(e.selectedOption[e.labelField],e.selectedOption,!0):e.placeholder),u=L(()=>{const i=e.selectedOption;if(i)return i[e.labelField]}),b=L(()=>e.multiple?!!(Array.isArray(e.selectedOptions)&&e.selectedOptions.length):e.selectedOption!==null);function z(){var i;const{value:h}=a;if(h){const{value:T}=f;T&&(T.style.width=`${h.offsetWidth}px`,e.maxTagCount!=="responsive"&&((i=C.value)===null||i===void 0||i.sync({showAllItemsBeforeCalculate:!1})))}}function I(){const{value:i}=m;i&&(i.style.display="none")}function E(){const{value:i}=m;i&&(i.style.display="inline-block")}te(ne(e,"active"),i=>{i||I()}),te(ne(e,"pattern"),()=>{e.multiple&&Me(z)});function O(i){const{onFocus:h}=e;h&&h(i)}function S(i){const{onBlur:h}=e;h&&h(i)}function K(i){const{onDeleteOption:h}=e;h&&h(i)}function W(i){const{onClear:h}=e;h&&h(i)}function _(i){const{onPatternInput:h}=e;h&&h(i)}function re(i){var h;(!i.relatedTarget||!(!((h=l.value)===null||h===void 0)&&h.contains(i.relatedTarget)))&&O(i)}function ie(i){var h;!((h=l.value)===null||h===void 0)&&h.contains(i.relatedTarget)||S(i)}function le(i){W(i)}function ae(){k.value=!0}function se(){k.value=!1}function ce(i){!e.active||!e.filterable||i.target!==f.value&&i.preventDefault()}function q(i){K(i)}const D=A(!1);function Y(i){if(i.key==="Backspace"&&!D.value&&!e.pattern.length){const{selectedOptions:h}=e;h!=null&&h.length&&q(h[h.length-1])}}let U=null;function de(i){const{value:h}=a;if(h){const T=i.target.value;h.textContent=T,z()}e.ignoreComposition&&D.value?U=i:_(i)}function ue(){D.value=!0}function J(){D.value=!1,e.ignoreComposition&&_(U),U=null}function he(i){var h;y.value=!0,(h=e.onPatternFocus)===null||h===void 0||h.call(e,i)}function fe(i){var h;y.value=!1,(h=e.onPatternBlur)===null||h===void 0||h.call(e,i)}function ve(){var i,h;if(e.filterable)y.value=!1,(i=d.value)===null||i===void 0||i.blur(),(h=f.value)===null||h===void 0||h.blur();else if(e.multiple){const{value:T}=r;T==null||T.blur()}else{const{value:T}=p;T==null||T.blur()}}function be(){var i,h,T;e.filterable?(y.value=!1,(i=d.value)===null||i===void 0||i.focus()):e.multiple?(h=r.value)===null||h===void 0||h.focus():(T=p.value)===null||T===void 0||T.focus()}function H(){const{value:i}=f;i&&(E(),i.focus())}function De(){const{value:i}=f;i&&i.blur()}function We(i){const{value:h}=c;h&&h.setTextContent(`+${i}`)}function He(){const{value:i}=s;return i}function je(){return f.value}let ge=null;function pe(){ge!==null&&window.clearTimeout(ge)}function Ge(){e.active||(pe(),ge=window.setTimeout(()=>{b.value&&(g.value=!0)},100))}function Ve(){pe()}function Ue(i){i||(pe(),g.value=!1)}te(b,i=>{i||(g.value=!1)}),Ie(()=>{Qt(()=>{const i=d.value;i&&(e.disabled?i.removeAttribute("tabindex"):i.tabIndex=y.value?-1:0)})}),rn(l,e.onResize);const{inlineThemeDisabled:Re}=e,Ee=L(()=>{const{size:i}=e,{common:{cubicBezierEaseInOut:h},self:{fontWeight:T,borderRadius:Ze,color:qe,placeholderColor:Ye,textColor:Je,paddingSingle:Qe,paddingMultiple:Xe,caretColor:et,colorDisabled:tt,textColorDisabled:nt,placeholderColorDisabled:ot,colorActive:rt,boxShadowFocus:it,boxShadowActive:lt,boxShadowHover:at,border:st,borderFocus:ct,borderHover:dt,borderActive:ut,arrowColor:ht,arrowColorDisabled:ft,loadingColor:vt,colorActiveWarning:bt,boxShadowFocusWarning:gt,boxShadowActiveWarning:pt,boxShadowHoverWarning:mt,borderWarning:Ct,borderFocusWarning:yt,borderHoverWarning:wt,borderActiveWarning:xt,colorActiveError:kt,boxShadowFocusError:zt,boxShadowActiveError:Pt,boxShadowHoverError:St,borderError:It,borderFocusError:$t,borderHoverError:Rt,borderActiveError:Et,clearColor:Ot,clearColorHover:Tt,clearColorPressed:_t,clearSize:At,arrowSize:Ft,[F("height",i)]:Bt,[F("fontSize",i)]:Lt}}=P.value,Q=ke(Qe),X=ke(Xe);return{"--n-bezier":h,"--n-border":st,"--n-border-active":ut,"--n-border-focus":ct,"--n-border-hover":dt,"--n-border-radius":Ze,"--n-box-shadow-active":lt,"--n-box-shadow-focus":it,"--n-box-shadow-hover":at,"--n-caret-color":et,"--n-color":qe,"--n-color-active":rt,"--n-color-disabled":tt,"--n-font-size":Lt,"--n-height":Bt,"--n-padding-single-top":Q.top,"--n-padding-multiple-top":X.top,"--n-padding-single-right":Q.right,"--n-padding-multiple-right":X.right,"--n-padding-single-left":Q.left,"--n-padding-multiple-left":X.left,"--n-padding-single-bottom":Q.bottom,"--n-padding-multiple-bottom":X.bottom,"--n-placeholder-color":Ye,"--n-placeholder-color-disabled":ot,"--n-text-color":Je,"--n-text-color-disabled":nt,"--n-arrow-color":ht,"--n-arrow-color-disabled":ft,"--n-loading-color":vt,"--n-color-active-warning":bt,"--n-box-shadow-focus-warning":gt,"--n-box-shadow-active-warning":pt,"--n-box-shadow-hover-warning":mt,"--n-border-warning":Ct,"--n-border-focus-warning":yt,"--n-border-hover-warning":wt,"--n-border-active-warning":xt,"--n-color-active-error":kt,"--n-box-shadow-focus-error":zt,"--n-box-shadow-active-error":Pt,"--n-box-shadow-hover-error":St,"--n-border-error":It,"--n-border-focus-error":$t,"--n-border-hover-error":Rt,"--n-border-active-error":Et,"--n-clear-size":At,"--n-clear-color":Ot,"--n-clear-color-hover":Tt,"--n-clear-color-pressed":_t,"--n-arrow-size":Ft,"--n-font-weight":T}}),j=Re?Se("internal-selection",L(()=>e.size[0]),Ee,e):void 0;return{mergedTheme:P,mergedClearable:R,mergedClsPrefix:n,rtlEnabled:o,patternInputFocused:y,filterablePlaceholder:w,label:u,selected:b,showTagsPanel:g,isComposing:D,counterRef:c,counterWrapperRef:s,patternInputMirrorRef:a,patternInputRef:f,selfRef:l,multipleElRef:r,singleElRef:p,patternInputWrapperRef:d,overflowRef:C,inputTagElRef:m,handleMouseDown:ce,handleFocusin:re,handleClear:le,handleMouseEnter:ae,handleMouseLeave:se,handleDeleteOption:q,handlePatternKeyDown:Y,handlePatternInputInput:de,handlePatternInputBlur:fe,handlePatternInputFocus:he,handleMouseEnterCounter:Ge,handleMouseLeaveCounter:Ve,handleFocusout:ie,handleCompositionEnd:J,handleCompositionStart:ue,onPopoverUpdateShow:Ue,focus:be,focusInput:H,blur:ve,blurInput:De,updateCounter:We,getCounter:He,getTail:je,renderLabel:e.renderLabel,cssVars:Re?void 0:Ee,themeClass:j==null?void 0:j.themeClass,onRender:j==null?void 0:j.onRender}},render(){const{status:e,multiple:n,size:t,disabled:o,filterable:a,maxTagCount:f,bordered:l,clsPrefix:r,ellipsisTagPopoverProps:p,onRender:d,renderTag:c,renderLabel:s}=this;d==null||d();const C=f==="responsive",m=typeof f=="number",g=C||m,y=v(Gt,null,{default:()=>v(Xt,{clsPrefix:r,loading:this.loading,showArrow:this.showArrow,showClear:this.mergedClearable&&this.selected,onClear:this.handleClear},{default:()=>{var P,R;return(R=(P=this.$slots).arrow)===null||R===void 0?void 0:R.call(P)}})});let k;if(n){const{labelField:P}=this,R=_=>v("div",{class:`${r}-base-selection-tag-wrapper`,key:_.value},c?c({option:_,handleClose:()=>{this.handleDeleteOption(_)}}):v(xe,{size:t,closable:!_.disabled,disabled:o,onClose:()=>{this.handleDeleteOption(_)},internalCloseIsButtonTag:!1,internalCloseFocusable:!1},{default:()=>s?s(_,!0):ee(_[P],_,!0)})),w=()=>(m?this.selectedOptions.slice(0,f):this.selectedOptions).map(R),u=a?v("div",{class:`${r}-base-selection-input-tag`,ref:"inputTagElRef",key:"__input-tag__"},v("input",Object.assign({},this.inputProps,{ref:"patternInputRef",tabindex:-1,disabled:o,value:this.pattern,autofocus:this.autofocus,class:`${r}-base-selection-input-tag__input`,onBlur:this.handlePatternInputBlur,onFocus:this.handlePatternInputFocus,onKeydown:this.handlePatternKeyDown,onInput:this.handlePatternInputInput,onCompositionstart:this.handleCompositionStart,onCompositionend:this.handleCompositionEnd})),v("span",{ref:"patternInputMirrorRef",class:`${r}-base-selection-input-tag__mirror`},this.pattern)):null,b=C?()=>v("div",{class:`${r}-base-selection-tag-wrapper`,ref:"counterWrapperRef"},v(xe,{size:t,ref:"counterRef",onMouseenter:this.handleMouseEnterCounter,onMouseleave:this.handleMouseLeaveCounter,disabled:o})):void 0;let z;if(m){const _=this.selectedOptions.length-f;_>0&&(z=v("div",{class:`${r}-base-selection-tag-wrapper`,key:"__counter__"},v(xe,{size:t,ref:"counterRef",onMouseenter:this.handleMouseEnterCounter,disabled:o},{default:()=>`+${_}`})))}const I=C?a?v(_e,{ref:"overflowRef",updateCounter:this.updateCounter,getCounter:this.getCounter,getTail:this.getTail,style:{width:"100%",display:"flex",overflow:"hidden"}},{default:w,counter:b,tail:()=>u}):v(_e,{ref:"overflowRef",updateCounter:this.updateCounter,getCounter:this.getCounter,style:{width:"100%",display:"flex",overflow:"hidden"}},{default:w,counter:b}):m&&z?w().concat(z):w(),E=g?()=>v("div",{class:`${r}-base-selection-popover`},C?w():this.selectedOptions.map(R)):void 0,O=g?Object.assign({show:this.showTagsPanel,trigger:"hover",overlap:!0,placement:"top",width:"trigger",onUpdateShow:this.onPopoverUpdateShow,theme:this.mergedTheme.peers.Popover,themeOverrides:this.mergedTheme.peerOverrides.Popover},p):null,K=(this.selected?!1:this.active?!this.pattern&&!this.isComposing:!0)?v("div",{class:`${r}-base-selection-placeholder ${r}-base-selection-overlay`},v("div",{class:`${r}-base-selection-placeholder__inner`},this.placeholder)):null,W=a?v("div",{ref:"patternInputWrapperRef",class:`${r}-base-selection-tags`},I,C?null:u,y):v("div",{ref:"multipleElRef",class:`${r}-base-selection-tags`,tabindex:o?void 0:0},I,y);k=v(Jt,null,g?v(nn,Object.assign({},O,{scrollable:!0,style:"max-height: calc(var(--v-target-height) * 6.6);"}),{trigger:()=>W,default:E}):W,K)}else if(a){const P=this.pattern||this.isComposing,R=this.active?!P:!this.selected,w=this.active?!1:this.selected;k=v("div",{ref:"patternInputWrapperRef",class:`${r}-base-selection-label`,title:this.patternInputFocused?void 0:Ae(this.label)},v("input",Object.assign({},this.inputProps,{ref:"patternInputRef",class:`${r}-base-selection-input`,value:this.active?this.pattern:"",placeholder:"",readonly:o,disabled:o,tabindex:-1,autofocus:this.autofocus,onFocus:this.handlePatternInputFocus,onBlur:this.handlePatternInputBlur,onInput:this.handlePatternInputInput,onCompositionstart:this.handleCompositionStart,onCompositionend:this.handleCompositionEnd})),w?v("div",{class:`${r}-base-selection-label__render-label ${r}-base-selection-overlay`,key:"input"},v("div",{class:`${r}-base-selection-overlay__wrapper`},c?c({option:this.selectedOption,handleClose:()=>{}}):s?s(this.selectedOption,!0):ee(this.label,this.selectedOption,!0))):null,R?v("div",{class:`${r}-base-selection-placeholder ${r}-base-selection-overlay`,key:"placeholder"},v("div",{class:`${r}-base-selection-overlay__wrapper`},this.filterablePlaceholder)):null,y)}else k=v("div",{ref:"singleElRef",class:`${r}-base-selection-label`,tabindex:this.disabled?void 0:0},this.label!==void 0?v("div",{class:`${r}-base-selection-input`,title:Ae(this.label),key:"input"},v("div",{class:`${r}-base-selection-input__content`},c?c({option:this.selectedOption,handleClose:()=>{}}):s?s(this.selectedOption,!0):ee(this.label,this.selectedOption,!0))):v("div",{class:`${r}-base-selection-placeholder ${r}-base-selection-overlay`,key:"placeholder"},v("div",{class:`${r}-base-selection-placeholder__inner`},this.placeholder)),y);return v("div",{ref:"selfRef",class:[`${r}-base-selection`,this.rtlEnabled&&`${r}-base-selection--rtl`,this.themeClass,e&&`${r}-base-selection--${e}-status`,{[`${r}-base-selection--active`]:this.active,[`${r}-base-selection--selected`]:this.selected||this.active&&this.pattern,[`${r}-base-selection--disabled`]:this.disabled,[`${r}-base-selection--multiple`]:this.multiple,[`${r}-base-selection--focus`]:this.focused}],style:this.cssVars,onClick:this.onClick,onMouseenter:this.handleMouseEnter,onMouseleave:this.handleMouseLeave,onKeydown:this.onKeydown,onFocusin:this.handleFocusin,onFocusout:this.handleFocusout,onMousedown:this.handleMouseDown},k,l?v("div",{class:`${r}-base-selection__border`}):null,l?v("div",{class:`${r}-base-selection__state-border`}):null)}});export{Gn as N,Hn as a,Vn as b,jn as c,$n as f,rn as u};
