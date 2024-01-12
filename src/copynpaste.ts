import showdialog from './debug';
import * as globals from './file-explorer';
// export const { resolve } = (window as any).__TAURI__.path;

export default function setsendpath(wt:string){
    globalThis.sendpath.push(wt)
    console.log(globalThis.sendpath)
    // console.log(globals.paste)
    globals.sync.textContent=`Sync (${sendpath.length})`
}
export function resetpaste(){
    globalThis.sendpath=[];
    globals.sync.textContent=`Sync`;
}

export function setpasteclick(){
    globals.sync.onclick= function () {
        (window as any).__TAURI__.invoke(
          "copynpaste",
          {
            // source: "/tmp/a/",
            source: (globalThis.sendpath),
            // dest: "/tmp/b/",
            dest: globals.pathInput.value,
          }
          ).
          then(
          (s:string)=>{
              showdialog("success"+s);
            
          }).
          catch(
          (e:string)=>{
          showdialog("failure"+e)
          });
        resetpaste();
      
              
      }
}