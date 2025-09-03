import{N as K,i as X}from"./Input-Xs7NyQR4.js";import{f as t,p as e,i as n,k as m,X as R,l as g,y as b,m as I,Y as L}from"./bootstrap-znN1ihFp.js";import{d as h,h as d,c as u}from"../jse/index-index-aJ4YWfRp.js";import"./use-locale-qGUlvW_R.js";import"./use-merged-state-T7vtHVBm.js";import"./Suffix-BnQD-wte.js";import"./Eye-D4UzjJ6m.js";const B=t("input-group",`
 display: inline-flex;
 width: 100%;
 flex-wrap: nowrap;
 vertical-align: bottom;
`,[e(">",[t("input",[e("&:not(:last-child)",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `),e("&:not(:first-child)",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 margin-left: -1px!important;
 `)]),t("button",[e("&:not(:last-child)",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `,[n("state-border, border",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `)]),e("&:not(:first-child)",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `,[n("state-border, border",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `)])]),e("*",[e("&:not(:last-child)",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `,[e(">",[t("input",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `),t("base-selection",[t("base-selection-label",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `),t("base-selection-tags",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `),n("box-shadow, border, state-border",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `)])])]),e("&:not(:first-child)",`
 margin-left: -1px!important;
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `,[e(">",[t("input",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `),t("base-selection",[t("base-selection-label",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `),t("base-selection-tags",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `),n("box-shadow, border, state-border",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `)])])])])])]),G={},j=h({name:"InputGroup",props:G,setup(r){const{mergedClsPrefixRef:o}=m(r);return R("-input-group",B,o),{mergedClsPrefix:o}},render(){const{mergedClsPrefix:r}=this;return d("div",{class:`${r}-input-group`},this.$slots)}}),k=t("input-group-label",`
 position: relative;
 user-select: none;
 -webkit-user-select: none;
 box-sizing: border-box;
 padding: 0 12px;
 display: inline-block;
 border-radius: var(--n-border-radius);
 background-color: var(--n-group-label-color);
 color: var(--n-group-label-text-color);
 font-size: var(--n-font-size);
 line-height: var(--n-height);
 height: var(--n-height);
 flex-shrink: 0;
 white-space: nowrap;
 transition: 
 color .3s var(--n-bezier),
 background-color .3s var(--n-bezier),
 box-shadow .3s var(--n-bezier);
`,[n("border",`
 position: absolute;
 left: 0;
 right: 0;
 top: 0;
 bottom: 0;
 border-radius: inherit;
 border: var(--n-group-label-border);
 transition: border-color .3s var(--n-bezier);
 `)]),$=Object.assign(Object.assign({},g.props),{size:{type:String,default:"medium"},bordered:{type:Boolean,default:void 0}}),E=h({name:"InputGroupLabel",props:$,setup(r){const{mergedBorderedRef:o,mergedClsPrefixRef:i,inlineThemeDisabled:s}=m(r),c=g("Input","-input-group-label",k,L,r,i),l=u(()=>{const{size:p}=r,{common:{cubicBezierEaseInOut:f},self:{groupLabelColor:v,borderRadius:x,groupLabelTextColor:z,lineHeight:C,groupLabelBorder:y,[b("fontSize",p)]:w,[b("height",p)]:P}}=c.value;return{"--n-bezier":f,"--n-group-label-color":v,"--n-group-label-border":y,"--n-border-radius":x,"--n-group-label-text-color":z,"--n-font-size":w,"--n-line-height":C,"--n-height":P}}),a=s?I("input-group-label",u(()=>r.size[0]),l,r):void 0;return{mergedClsPrefix:i,mergedBordered:o,cssVars:s?void 0:l,themeClass:a==null?void 0:a.themeClass,onRender:a==null?void 0:a.onRender}},render(){var r,o,i;const{mergedClsPrefix:s}=this;return(r=this.onRender)===null||r===void 0||r.call(this),d("div",{class:[`${s}-input-group-label`,this.themeClass],style:this.cssVars},(i=(o=this.$slots).default)===null||i===void 0?void 0:i.call(o),this.mergedBordered?d("div",{class:`${s}-input-group-label__border`}):null)}});export{K as NInput,j as NInputGroup,E as NInputGroupLabel,$ as inputGroupLabelProps,G as inputGroupProps,X as inputProps};
