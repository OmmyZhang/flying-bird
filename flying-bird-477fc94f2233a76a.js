let U=0,Y=`string`,W=1,_=`Object`,S=`utf-8`,Q=null,X=`number`,R=`undefined`,a0=4,Z=`function`,O=128,N=Array,T=Error,$=JSON,a1=Object,V=Uint8Array,P=undefined;var H=(async(a,b)=>{if(typeof Response===Z&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===Z){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var q=(a=>{const b=typeof a;if(b==X||b==`boolean`||a==Q){return `${a}`};if(b==Y){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==Q){return `Symbol`}else{return `Symbol(${b})`}};if(b==Z){const b=a.name;if(typeof b==Y&&b.length>U){return `Function(${b})`}else{return `Function`}};if(N.isArray(a)){const b=a.length;let c=`[`;if(b>U){c+=q(a[U])};for(let d=W;d<b;d++){c+=`, `+ q(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>W){d=c[W]}else{return toString.call(a)};if(d==_){try{return `Object(`+ $.stringify(a)+ `)`}catch(a){return _}};if(a instanceof T){return `${a.name}: ${a.message}\n${a.stack}`};return d});var w=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__ha482bea2b7bc66d8(b,c)});var J=((a,b)=>{});var B=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd93f4746c8dbe6c8(b,c,k(d))});var F=((a,b)=>{a=a>>>U;const c=E();const d=c.subarray(a/a0,a/a0+ b);const e=[];for(let a=U;a<d.length;a++){e.push(f(d[a]))};return e});var k=(a=>{if(d===b.length)b.push(b.length+ W);const c=d;d=b[c];b[c]=a;return c});var f=(a=>{const b=c(a);e(a);return b});var M=(async(b)=>{if(a!==P)return a;if(typeof b===R){b=new URL(`flying-bird-477fc94f2233a76a_bg.wasm`,import.meta.url)};const c=I();if(typeof b===Y||typeof Request===Z&&b instanceof Request||typeof URL===Z&&b instanceof URL){b=fetch(b)};J(c);const {instance:d,module:e}=await H(await b,c);return K(d,e)});var L=(b=>{if(a!==P)return a;const c=I();J(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return K(d,b)});function C(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(k(b))}}var K=((b,c)=>{a=b.exports;M.__wbindgen_wasm_module=c;m=Q;o=Q;D=Q;h=Q;a.__wbindgen_start();return a});var I=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==W){b.a=U;return !0};const c=!1;return c});b.wbg.__wbindgen_string_new=((a,b)=>{const c=j(a,b);return k(c)});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===X?d:P;n()[a/8+ W]=l(e)?U:e;p()[a/a0+ U]=!l(e)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return k(b)});b.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const d=c(b).__yew_subtree_cache_key;p()[a/a0+ W]=l(d)?U:d;p()[a/a0+ U]=!l(d)});b.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const d=c(b).__yew_subtree_id;p()[a/a0+ W]=l(d)?U:d;p()[a/a0+ U]=!l(d)});b.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{c(a).__yew_subtree_id=b>>>U});b.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>U});b.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const d=c(b).__yew_listener_id;p()[a/a0+ W]=l(d)?U:d;p()[a/a0+ U]=!l(d)});b.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{c(a).__yew_listener_id=b>>>U});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new T();return k(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=r;p()[b/a0+ W]=g;p()[b/a0+ U]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(j(b,c))}finally{a.__wbindgen_free(d,e,W)}});b.wbg.__wbg_clearInterval_7f51e4380e64c6c5=(a=>{const b=clearInterval(f(a));return k(b)});b.wbg.__wbg_setInterval_e227d4d8a9d44d66=function(){return C(((a,b)=>{const d=setInterval(c(a),b);return k(d)}),arguments)};b.wbg.__wbg_queueMicrotask_118eeb525d584d9a=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_26a89c14c53809c0=(a=>{const b=c(a).queueMicrotask;return k(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===Z;return b});b.wbg.__wbg_error_a526fb08a0205972=((b,c)=>{var d=F(b,c).slice();a.__wbindgen_free(b,c*a0,a0);console.error(...d)});b.wbg.__wbg_crypto_d05b68a3572bb8ca=(a=>{const b=c(a).crypto;return k(b)});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==Q;return d});b.wbg.__wbg_process_b02b3570280d0366=(a=>{const b=c(a).process;return k(b)});b.wbg.__wbg_versions_c1cb42213cedf0f5=(a=>{const b=c(a).versions;return k(b)});b.wbg.__wbg_node_43b1089f407e4ec2=(a=>{const b=c(a).node;return k(b)});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===Y;return b});b.wbg.__wbg_msCrypto_10fc94afee92bd76=(a=>{const b=c(a).msCrypto;return k(b)});b.wbg.__wbg_require_9a7e0f667ead4995=function(){return C((()=>{const a=module.require;return k(a)}),arguments)};b.wbg.__wbg_randomFillSync_b70ccbdf4926a99d=function(){return C(((a,b)=>{c(a).randomFillSync(f(b))}),arguments)};b.wbg.__wbg_getRandomValues_7e42b4fb8779dc6d=function(){return C(((a,b)=>{c(a).getRandomValues(c(b))}),arguments)};b.wbg.__wbg_instanceof_Window_99dc9805eaa2614b=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5257b70811e953c0=(a=>{const b=c(a).document;return l(b)?U:k(b)});b.wbg.__wbg_innerWidth_0bae627f0302b204=function(){return C((a=>{const b=c(a).innerWidth;return k(b)}),arguments)};b.wbg.__wbg_innerHeight_dc4c81e04e8bc294=function(){return C((a=>{const b=c(a).innerHeight;return k(b)}),arguments)};b.wbg.__wbg_setonresize_862d01deb25a20bb=((a,b)=>{c(a).onresize=c(b)});b.wbg.__wbg_localStorage_318b1c4f106a46f9=function(){return C((a=>{const b=c(a).localStorage;return l(b)?U:k(b)}),arguments)};b.wbg.__wbg_documentElement_a030fb9a26624716=(a=>{const b=c(a).documentElement;return l(b)?U:k(b)});b.wbg.__wbg_body_3eb73da919b867a1=(a=>{const b=c(a).body;return l(b)?U:k(b)});b.wbg.__wbg_createElement_1a136faad4101f43=function(){return C(((a,b,d)=>{const e=c(a).createElement(j(b,d));return k(e)}),arguments)};b.wbg.__wbg_createElementNS_d47e0c50fa2904e0=function(){return C(((a,b,d,e,f)=>{const g=c(a).createElementNS(b===U?P:j(b,d),j(e,f));return k(g)}),arguments)};b.wbg.__wbg_createTextNode_dbdd908f92bae1b1=((a,b,d)=>{const e=c(a).createTextNode(j(b,d));return k(e)});b.wbg.__wbg_instanceof_Element_f614cf57d4316979=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_namespaceURI_0819c2800784a176=((b,d)=>{const e=c(d).namespaceURI;var f=l(e)?U:u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=r;p()[b/a0+ W]=g;p()[b/a0+ U]=f});b.wbg.__wbg_setinnerHTML_99deeacfff0ae4cc=((a,b,d)=>{c(a).innerHTML=j(b,d)});b.wbg.__wbg_outerHTML_69934f9195df65af=((b,d)=>{const e=c(d).outerHTML;const f=u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=r;p()[b/a0+ W]=g;p()[b/a0+ U]=f});b.wbg.__wbg_removeAttribute_5c264e727b67dbdb=function(){return C(((a,b,d)=>{c(a).removeAttribute(j(b,d))}),arguments)};b.wbg.__wbg_requestFullscreen_86cc33835b4db656=function(){return C((a=>{c(a).requestFullscreen()}),arguments)};b.wbg.__wbg_setAttribute_0918ea45d5a1c663=function(){return C(((a,b,d,e,f)=>{c(a).setAttribute(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_focus_623326ec4eefd224=function(){return C((a=>{c(a).focus()}),arguments)};b.wbg.__wbg_target_791826e938c3e308=(a=>{const b=c(a).target;return l(b)?U:k(b)});b.wbg.__wbg_bubbles_f0783dc095f8e220=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_cancelBubble_191799b8e0ab3254=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_d94a39b8c8f6eed1=(a=>{const b=c(a).composedPath();return k(b)});b.wbg.__wbg_addEventListener_1b158e9e95e0ab00=function(){return C(((a,b,d,e,f)=>{c(a).addEventListener(j(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_log_9dfb3879776dd797=(a=>{console.log(c(a))});b.wbg.__wbg_setwidth_05075fb6b4cc720e=((a,b)=>{c(a).width=b>>>U});b.wbg.__wbg_setheight_7e0e88a922100d8c=((a,b)=>{c(a).height=b>>>U});b.wbg.__wbg_getContext_1daf9aba3e114993=function(){return C(((a,b,d,e)=>{const f=c(a).getContext(j(b,d),c(e));return l(f)?U:k(f)}),arguments)};b.wbg.__wbg_value_ab23a75318ea828f=((b,d)=>{const e=c(d).value;const f=u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=r;p()[b/a0+ W]=g;p()[b/a0+ U]=f});b.wbg.__wbg_setvalue_918a8ae77531a942=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_getItem_f7e7a061bbdabefe=function(){return C(((b,d,e,f)=>{const g=c(d).getItem(j(e,f));var h=l(g)?U:u(g,a.__wbindgen_malloc,a.__wbindgen_realloc);var i=r;p()[b/a0+ W]=i;p()[b/a0+ U]=h}),arguments)};b.wbg.__wbg_setItem_2b72ddf192083111=function(){return C(((a,b,d,e,f)=>{c(a).setItem(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_setchecked_3b12f3d602a63e47=((a,b)=>{c(a).checked=b!==U});b.wbg.__wbg_value_c93cb4b4d352228e=((b,d)=>{const e=c(d).value;const f=u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=r;p()[b/a0+ W]=g;p()[b/a0+ U]=f});b.wbg.__wbg_setvalue_9bd3f93b3864ddbf=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_data_a1416834d6367ac7=((b,d)=>{const e=c(d).data;const f=G(e,a.__wbindgen_malloc);const g=r;p()[b/a0+ W]=g;p()[b/a0+ U]=f});b.wbg.__wbg_setcurrentTime_1d19168c4fe9fbea=((a,b)=>{c(a).currentTime=b});b.wbg.__wbg_setvolume_61a9621fdfe696e1=((a,b)=>{c(a).volume=b});b.wbg.__wbg_pause_e2f9ecbef742a554=function(){return C((a=>{c(a).pause()}),arguments)};b.wbg.__wbg_play_148e53a017c4c640=function(){return C((a=>{const b=c(a).play();return k(b)}),arguments)};b.wbg.__wbg_instanceof_HtmlImageElement_0062f0e2d7a0cb87=(a=>{let b;try{b=c(a) instanceof HTMLImageElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_parentNode_f3957fdd408a62f7=(a=>{const b=c(a).parentNode;return l(b)?U:k(b)});b.wbg.__wbg_parentElement_86a7612dde875ba9=(a=>{const b=c(a).parentElement;return l(b)?U:k(b)});b.wbg.__wbg_childNodes_75d3da5f3a7bb985=(a=>{const b=c(a).childNodes;return k(b)});b.wbg.__wbg_lastChild_8f7b6f3825115eff=(a=>{const b=c(a).lastChild;return l(b)?U:k(b)});b.wbg.__wbg_nextSibling_13e9454ef5323f1a=(a=>{const b=c(a).nextSibling;return l(b)?U:k(b)});b.wbg.__wbg_setnodeValue_8656e865e9b11bbb=((a,b,d)=>{c(a).nodeValue=b===U?P:j(b,d)});b.wbg.__wbg_textContent_efe8338af53ddf62=((b,d)=>{const e=c(d).textContent;var f=l(e)?U:u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=r;p()[b/a0+ W]=g;p()[b/a0+ U]=f});b.wbg.__wbg_cloneNode_80501c66ab115588=function(){return C((a=>{const b=c(a).cloneNode();return k(b)}),arguments)};b.wbg.__wbg_insertBefore_882082ef4c5d7766=function(){return C(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_removeChild_14b08321b677677a=function(){return C(((a,b)=>{const d=c(a).removeChild(c(b));return k(d)}),arguments)};b.wbg.__wbg_instanceof_ShadowRoot_cb6366cb0956ce29=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_99e27ed8897850f2=(a=>{const b=c(a).host;return k(b)});b.wbg.__wbg_setstrokeStyle_7459000441c14f13=((a,b)=>{c(a).strokeStyle=c(b)});b.wbg.__wbg_setfillStyle_0284526726395e47=((a,b)=>{c(a).fillStyle=c(b)});b.wbg.__wbg_setlineWidth_7d11c2d417a3d73d=((a,b)=>{c(a).lineWidth=b});b.wbg.__wbg_drawImage_7c1321d754212efe=function(){return C(((a,b,d,e,f,g)=>{c(a).drawImage(c(b),d,e,f,g)}),arguments)};b.wbg.__wbg_beginPath_317e9c4bf21bdeaa=(a=>{c(a).beginPath()});b.wbg.__wbg_stroke_31f20cb8d85be0b9=(a=>{c(a).stroke()});b.wbg.__wbg_getImageData_23ae615d32adfe2f=function(){return C(((a,b,d,e,f)=>{const g=c(a).getImageData(b,d,e,f);return k(g)}),arguments)};b.wbg.__wbg_lineTo_3fcfab7d5f3282a0=((a,b,d)=>{c(a).lineTo(b,d)});b.wbg.__wbg_moveTo_85c1057e0b74ac9d=((a,b,d)=>{c(a).moveTo(b,d)});b.wbg.__wbg_fillRect_7f9e32d06a574988=((a,b,d,e,f)=>{c(a).fillRect(b,d,e,f)});b.wbg.__wbg_restore_66f03665d18b1fc4=(a=>{c(a).restore()});b.wbg.__wbg_save_2ea393c01ff5efb9=(a=>{c(a).save()});b.wbg.__wbg_rotate_0f52d6b60535f069=function(){return C(((a,b)=>{c(a).rotate(b)}),arguments)};b.wbg.__wbg_translate_b5617b00590093a8=function(){return C(((a,b,d)=>{c(a).translate(b,d)}),arguments)};b.wbg.__wbg_get_c43534c00f382c8a=((a,b)=>{const d=c(a)[b>>>U];return k(d)});b.wbg.__wbg_length_d99b680fd68bf71b=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_5859b6d41c6fe9f7=((a,b)=>{const c=new Function(j(a,b));return k(c)});b.wbg.__wbg_call_a79f1973a4f07d5e=function(){return C(((a,b)=>{const d=c(a).call(c(b));return k(d)}),arguments)};b.wbg.__wbg_new_87d841e70661f6e9=(()=>{const a=new a1();return k(a)});b.wbg.__wbg_self_086b5302bcafb962=function(){return C((()=>{const a=self.self;return k(a)}),arguments)};b.wbg.__wbg_window_132fa5d7546f1de5=function(){return C((()=>{const a=window.window;return k(a)}),arguments)};b.wbg.__wbg_globalThis_e5f801a37ad7d07b=function(){return C((()=>{const a=globalThis.globalThis;return k(a)}),arguments)};b.wbg.__wbg_global_f9a61fce4af6b7c1=function(){return C((()=>{const a=global.global;return k(a)}),arguments)};b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===P;return b});b.wbg.__wbg_from_a663e01d8dab8e44=(a=>{const b=N.from(c(a));return k(b)});b.wbg.__wbg_call_f6a2bc58c19c53c6=function(){return C(((a,b,d)=>{const e=c(a).call(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_is_a5728dbfb61c82cd=((a,b)=>{const d=a1.is(c(a),c(b));return d});b.wbg.__wbg_resolve_97ecd55ee839391b=(a=>{const b=Promise.resolve(c(a));return k(b)});b.wbg.__wbg_then_7aeb7c5f1536640f=((a,b)=>{const d=c(a).then(c(b));return k(d)});b.wbg.__wbg_buffer_5d1b598a01b41a42=(a=>{const b=c(a).buffer;return k(b)});b.wbg.__wbg_newwithbyteoffsetandlength_d695c7957788f922=((a,b,d)=>{const e=new V(c(a),b>>>U,d>>>U);return k(e)});b.wbg.__wbg_new_ace717933ad7117f=(a=>{const b=new V(c(a));return k(b)});b.wbg.__wbg_set_74906aa30864df5a=((a,b,d)=>{c(a).set(c(b),d>>>U)});b.wbg.__wbg_newwithlength_728575f3bba9959b=(a=>{const b=new V(a>>>U);return k(b)});b.wbg.__wbg_subarray_7f7a652672800851=((a,b,d)=>{const e=c(a).subarray(b>>>U,d>>>U);return k(e)});b.wbg.__wbg_parse_06816e879d29d4df=function(){return C(((a,b)=>{const c=$.parse(j(a,b));return k(c)}),arguments)};b.wbg.__wbg_set_37a50e901587b477=function(){return C(((a,b,d)=>{const e=Reflect.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=q(c(d));const f=u(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=r;p()[b/a0+ W]=g;p()[b/a0+ U]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new T(j(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return k(b)});b.wbg.__wbindgen_closure_wrapper303=((a,b,c)=>{const d=v(a,b,116,w);return k(d)});b.wbg.__wbindgen_closure_wrapper574=((a,b,c)=>{const d=x(a,b,209,A);return k(d)});b.wbg.__wbindgen_closure_wrapper714=((a,b,c)=>{const d=v(a,b,259,B);return k(d)});return b});var p=(()=>{if(o===Q||o.byteLength===U){o=new Int32Array(a.memory.buffer)};return o});var l=(a=>a===P||a===Q);var c=(a=>b[a]);var G=((a,b)=>{const c=b(a.length*W,W)>>>U;i().set(a,c/W);r=a.length;return c});var E=(()=>{if(D===Q||D.byteLength===U){D=new Uint32Array(a.memory.buffer)};return D});var A=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_ref__h72de7da5e70aec0e(c,d,z(e))}finally{b[y++]=P}});var n=(()=>{if(m===Q||m.byteLength===U){m=new Float64Array(a.memory.buffer)};return m});var x=((b,c,d,e)=>{const f={a:b,b:c,cnt:W,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===U){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=U}}};g.original=f;return g});var e=(a=>{if(a<132)return;b[a]=d;d=a});var v=((b,c,d,e)=>{const f={a:b,b:c,cnt:W,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=U;try{return e(c,f.b,...b)}finally{if(--f.cnt===U){a.__wbindgen_export_2.get(f.dtor)(c,f.b)}else{f.a=c}}};g.original=f;return g});var u=((a,b,c)=>{if(c===P){const c=s.encode(a);const d=b(c.length,W)>>>U;i().subarray(d,d+ c.length).set(c);r=c.length;return d};let d=a.length;let e=b(d,W)>>>U;const f=i();let g=U;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==U){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,W)>>>U;const b=i().subarray(e+ g,e+ d);const f=t(a,b);g+=f.written};r=g;return e});var i=(()=>{if(h===Q||h.byteLength===U){h=new V(a.memory.buffer)};return h});var j=((a,b)=>{a=a>>>U;return g.decode(i().subarray(a,a+ b))});var z=(a=>{if(y==W)throw new T(`out of js stack`);b[--y]=a;return y});let a;const b=new N(O).fill(P);b.push(P,Q,!0,!1);let d=b.length;const g=typeof TextDecoder!==R?new TextDecoder(S,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw T(`TextDecoder not available`)}};if(typeof TextDecoder!==R){g.decode()};let h=Q;let m=Q;let o=Q;let r=U;const s=typeof TextEncoder!==R?new TextEncoder(S):{encode:()=>{throw T(`TextEncoder not available`)}};const t=typeof s.encodeInto===Z?((a,b)=>s.encodeInto(a,b)):((a,b)=>{const c=s.encode(a);b.set(c);return {read:a.length,written:c.length}});let y=O;let D=Q;export default M;export{L as initSync}