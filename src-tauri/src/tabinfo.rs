use serde::Serialize;
use tauri::{Window, State, Manager};

use crate::{appstate::AppStateStore, list_files, sendtofrontend::loadmarks};


#[derive(Clone,Debug,Serialize)]
pub struct tabinfo{
    pub id:String,
    pub path:String,
    pub ff:String,
    pub tabname:String,
    pub history:Vec<String>
}

#[derive(Clone,Debug,Serialize)]
pub struct tab{
    pub path:String,
    pub focusfolder:String,
    pub history:Vec<String>,
    pub windowname:String
}

#[tauri::command]
pub async fn load_tab(windowname:&str,oid:String,window: Window, state: State<'_, AppStateStore>) -> Result<(), String> {

  let (path,_,_)=state.gettab(&oid,windowname.to_string().clone());
  // for i in state.gettabsfromwinlabel(&windowname.to_string()){
  //   if()
  // }
  println!("loadtab {} in window {}", oid.clone(),windowname.clone());

  list_files(windowname.to_string(),oid, path, "newtab".to_string(), window, state).await?;
Ok(())
}
#[tauri::command]
pub async fn closetab(windowname:&str,id:String,window: Window,state: State<'_, AppStateStore>)->Result<(),()>{
  state.removetab(id,windowname.to_string());
  let app_handle = window.app_handle();
  // app_handle.emit_to(
  //   windowname,
  //   "list-tabs",
  //   serde_json::to_string(&state.gettabs()).unwrap(),
  // )
  // .map_err(|e| e.to_string()).unwrap();
  Ok(())
}
// #[tauri::command]
// pub async fn listtabs(windowname:&str,window: Window,state: State<'_, AppStateStore>)->Result<(),()>{
//   let app_handle = window.app_handle();
//   // println!("{:?}",state);
//   app_handle.emit_to(
//     windowname,
//     "list-tabs",
//     serde_json::to_string(&state.gettabs()).unwrap(),
//   )
//   .map_err(|e| e.to_string()).unwrap();
// loadmarks(windowname,&app_handle,serde_json::to_string(&state.getmarks()).unwrap());
// Ok(())
// }
#[tauri::command]
pub async fn newtab(windowname:&str,oid:String,path:String,ff:String,window: Window,state: State<'_, AppStateStore>)->Result<(),()>{
  state.addtab(oid.clone(), path.clone(), ff.clone(),windowname.to_string());

  println!("added tab {} to window {} for path {}",oid,windowname,path);
  
  // listtabs(windowname,window, state).await;
  Ok(())
}