#![warn(clippy::disallowed_types)]
use filesize::PathExt;
use prefstore::getallcustomwithin;
use std::collections::{HashSet, HashMap};
use std::mem::{self};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI8, Ordering, AtomicI16};
// use std::sync::mpsc::{Sender, Receiver};
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::sync::{RwLock, Mutex, Arc};
use crate::trie::TrieNode;
use crate::{tabinfo::*};
use crate::bookmarks::*;
#[derive(Clone,Debug)]
// Use the gio crate
// use gio::prelude::*;
pub struct cachestore{
    pub size:u64,
    pub expirytime: u64,
}


#[derive(Debug)]
// // A function that takes a path to a file or directory and returns its size in bytes
// fn file_size(path: &str) -> u64 {
//     // Create a GFile object from the path
//     let file = gio::File::for_path(path);
//     // Call g_file_query_info with the file and some attributes and flags
//     let info = file.query_info(
//         gio::FILE_ATTRIBUTE_STANDARD_SIZE,
//         gio::FileQueryInfoFlags::NOFOLLOW_SYMLINKS,
//         None::<&gio::Cancellable>,
//     )
//     .unwrap();
//     // Get the size of the file from info
//     info.size() as u64
// }
// // Another function that takes a path to a file and returns its size in bytes
// fn file_size2(path: &str) -> u64 {
//     // Open the file in read-only mode
//     let file = File::open(path).unwrap();
//     // Get an iterator over the bytes of the file
//     let bytes = file.bytes();
//     // Count the number of bytes and return it
//     bytes.count() as u64
// }
// Define a struct that holds the cache and the expiration time
pub struct AppStateStore {
    pub cstore:RwLock<FxHashMap<String,cachestore>>,
    pub nosize:RwLock<bool>,
    pub filesetcollection:RwLock<HashMap<String,i32>>,
    pub showfolderchildcount:RwLock<bool>,
    pub loadsearchlist:RwLock<bool>,
    tabs:RwLock<FxHashMap<String,tab>>,
    pub expiration:Duration,
    pub whichthread:Arc<AtomicI8>,
    pub searchcounter:Arc<AtomicI16>,
    bookmarks:RwLock<HashSet<marks>>,
    messagetothread:RwLock<String>,
    recents:Vec<String>,
    pub aborted:Arc<Mutex<bool>>,
    filechanged:Arc<Mutex<bool>>,
    pub searchtry:Arc<Mutex<HashSet<String>>>,
    pub st:Arc<Mutex<TrieNode>>,
    pub stl:Arc<Mutex<FxHashMap<String,HashSet<String>>>>,
    pub process_count: Arc<Mutex<i32>>,
    pub buttonnames:HashMap<String,String>
    // tx: Mutex<Option<Sender<String>>>,
    // rx: Mutex<Option<Receiver<String>>>,
    // tx:(RwLock<Sender<String>>),
    // rx:RwLock<Receiver<String>>
    // app_handle:AppHandle
    // size:usize
}
#[derive(Debug)]
pub enum wThread {
    None,
    Populating,
    Searching,
    Listing
}
pub fn set_enum_value(atomic_enum: &AtomicI8, value: wThread) {
    let mapped_value = match value {
        wThread::None => 0,
        wThread::Populating => 1,
        wThread::Searching => 2,
        wThread::Listing => 3,
    };


    atomic_enum.store(mapped_value, Ordering::SeqCst);
}
pub fn get_enum_value(atomic_enum: &AtomicI8)->wThread {
    match atomic_enum.load(Ordering::SeqCst) {
        0 => wThread::None,
        1 => wThread::Populating ,
        2 => wThread::Searching,
        3 => wThread::Listing,
        _ => wThread::None
    }
}

use crate::{dirsize, sizeunit};
impl AppStateStore {
    pub fn new(expiration: u64) -> Self {
        // let (tx, rx) = mpsc::channel::<String>();

        Self {
            // Wrap the cache in a RwLock
            cstore:RwLock::new(FxHashMap::default()),
            whichthread:Arc::new(AtomicI8::new(0)),
            searchcounter:Arc::new(AtomicI16::new(0)),
            nosize:RwLock::new(true),
            filesetcollection:RwLock::new(HashMap::new()),
            showfolderchildcount:RwLock::new(false),
            loadsearchlist:RwLock::new(false),
            tabs:RwLock::new(FxHashMap::default()),
            expiration:Duration::from_secs(expiration),
            bookmarks:RwLock::new(HashSet::new()),
            messagetothread:RwLock::new(String::new()),
            recents:Vec::new(),
            aborted:Arc::new(Mutex::new(false)),
            filechanged:Arc::new(Mutex::new(false)),
            searchtry:Arc::new(Mutex::new(HashSet::new())),
            st:Arc::new(Mutex::new(TrieNode::new())),
            stl:Arc::new(Mutex::new(FxHashMap::default())),
            process_count: Arc::new(Mutex::new(0)),
            buttonnames: {
               let mut buttonnames=HashMap::new();
                for (i,j) in getallcustomwithin("filedime", "custom_scripts","fds"){
                buttonnames.insert(i.clone(),j.clone());
              }
              buttonnames
            }
            // tx:Mutex::new(Some(tx)),
            // rx:Mutex::new(Some(rx))
            // tx:RwLock::new(Some(tx)),
            // rx:RwLock::new(Some(rx))
            // app_handle: apphandle
            // size:0
        }
    }
    pub fn addmark(&self,path:String){
        self.bookmarks.write().unwrap().insert(marks { path: path.clone(), name: PathBuf::from(path).file_stem().unwrap().to_string_lossy().to_string() });
    }
    pub fn addtab(&self,id:String,path:String,mut ff:String,windowname:String){
        println!("{}---{}---{}",id,path,ff);
        // let id=format!("{}",(id.parse::<i32>().unwrap()+1));
        let mut tabhist=Vec::new();
        match(self.tabs.read().unwrap().get(&(windowname.clone()+"."+&id))){
            Some(tabi)=>{
                tabhist=tabi.history.clone();
                if(
                    ff!="back" 
                    && 
                    ff!="newtab"
                ){
                    let tabprevurl=tabi.path.clone();
                    tabhist.push(tabprevurl);
                }
                else{
                    ff="".to_string();
                }
            },
            None=>{

            }
        }

        let mut tabs=self.tabs.write().unwrap();
        tabs.insert((windowname.clone()+"."+&id),tab {
                path: path, 
                focusfolder: ff,
                history: tabhist,
                windowname:windowname
            }
        );
    }
    pub fn removetab(&self,id:String,windowname:String){
        // println!("{}---{}---{}",id,path,ff);
        
        let mut tabs=self.tabs.write().unwrap();
        tabs.remove(&(windowname+"."+&id));
    }
    pub fn removemark(&self,path:String){
        // println!("{}---{}---{}",id,path,ff);
        
        let mut marks=self.bookmarks.write().unwrap();
        marks.retain(|s| s.path != path);
    }
    pub fn getmarks(&self)->HashSet<marks>{
        self.bookmarks.read().unwrap().clone()
    }
    // pub fn gettabs(&self)->Vec<tabinfo>{
    //     let mut tvecs=Vec::new();
    //     let binding = self.tabs.read().unwrap();
    //     let mut hi=binding.iter();
    //     while let Some(ei)=hi.next(){
    //         tvecs.push(tabinfo{
    //             id:ei.0.clone(),
    //             path:ei.1.path.clone(),
    //             ff:ei.1.focusfolder.clone(),
    //             tabname:{
    //                 if let Some(h)=PathBuf::from(ei.1.path.clone()).file_stem(){
    //                     h.to_string_lossy().to_string()
    //                 }
    //                 else{
    //                     "".to_string()
    //                 }
    //             },
    //             history:ei.1.history.clone()
    //         })
    //     }
    //     tvecs
    //     // self.tabs.read().unwrap().clone()
    // }
    
    pub fn getlasthistory(&self,id:String,windowname:String)->Option<String>{
        let gtab= self.tabs.read().unwrap();
        let path=gtab.get(&(windowname.clone()+"."+&id)).unwrap().path.clone();
        let ff=gtab.get(&(windowname.clone()+"."+&id)).unwrap().focusfolder.clone();
        let mut tabhist=gtab.get(&(windowname.clone()+"."+&id)).unwrap().history.clone();
        let mut windowname=gtab.get(&(windowname.clone()+"."+&id)).unwrap().windowname.clone();
        let lastval=tabhist.pop();
        
        drop(gtab);
        
        
        
        let mut tabs= self.tabs.write().unwrap();
        // let mut tabhist=tabs.get(&id).unwrap().history;
        tabs.insert(
            (windowname.clone()+"."+&id),
            tab 
            {
                path: path, 
                focusfolder: ff,
                history: tabhist,
                windowname:windowname
            }
        );
        lastval

    }
    pub fn gettab(&self,id:&String,windowname:String)->(String,String,Vec<String>){
        let tab= self.tabs.read().unwrap().get(&(windowname+"."+id)).unwrap().clone();
        return (
            tab.path,
            tab.focusfolder,
            tab.history
        )
    }
    pub fn gettabfromwinlabel(&self,winlabel:&String)->Option<(String,String)>{
        let elements=self.tabs.read().unwrap().clone();
        for (id,others) in elements{
            if(others.windowname==winlabel.clone()){
                return Some((others.path,id))
            }
        }
        None
    }

    pub fn gettabsfromwinlabel(&self,winlabel:&String)->Vec<String>
    {
        let elements=self.tabs.read().unwrap().clone();
        let mut tabsofwindow=vec![];
        for (id,others) in elements{
            if(others.windowname==winlabel.clone()){
                tabsofwindow.push(id);
            // return Some((others.path,id))
            }
        }
        tabsofwindow
    }
    pub fn getactivepath(&self,id:&str,windowname:String)->(String,String,Vec<String>){
        let tab= self.tabs.read().unwrap().get(&(windowname+"."+id)).unwrap().clone();
        return (
            tab.path,
            tab.focusfolder,
            tab.history
        )
    }
    
    
// pub fn find_size(&self, path: &str) -> u64 {
//     // return 0 as u64;
//     let cstore=self.cstore.read().unwrap();

//     // let k=0;
//     // if(k==0){
//     //     return 0;
//     // }
//     // Use a single read lock guard to access the cache
//     let cache = cstore;

//     if let Some(cstore) = cache.get(path) {
//         let size=cstore.size;
//         let expirytime=cstore.expirytime;
//         let now = SystemTime::now();
//         let duration = now.duration_since(UNIX_EPOCH).unwrap();
//         let nowtime = duration.as_secs();
//         // Use the same lock guard to get the expiry time
//         // if let Some(expirytime) = cache.get(&("expiry_".to_string() + &path.to_string())) {
//             if nowtime < expirytime {
//                 return size;
//             } else {
//                 println!("expired")
//             }
//         // }
//     }

//     // Drop the read lock guard before acquiring a write lock guard
//     drop(cache);
//     let entry_path = Path::new(path);
//     if(entry_path.is_file()){

//         return entry_path.size_on_disk().unwrap_or(0)
//     }
//     if !entry_path.is_dir(){
//         return 0;
//     }
//     let nosize=self.nosize.read().unwrap();
//     if(*nosize){
//         return 0
//     }
//     let mut size = {
//             // 0 as u64
//             dirsize::dir_size(
//                 &entry_path.as_os_str().to_os_string().to_string_lossy().to_string(),
//                 self,
//             )

//         };

//     if(size!=0){
//         // Use a single write lock guard to update the cache
//         let  cstore=self.cstore.write().unwrap();
    
//         let mut cache = cstore;
        
        
//         let now = SystemTime::now();
        
//         let later = now + (self.expiration);
        
//         let duration = later.duration_since(UNIX_EPOCH).unwrap();
        
//         let expirytime = duration.as_secs();
//         // cache.insert("expiry_".to_string() + &path.to_string(), expirytime);
//         cache.insert(path.to_string(), cachestore { size: size, expirytime: expirytime });
        
//     }
    
//     // Add the size of the key and the value to the total
//     // self.size += mem::size_of_val(&path.to_string());
//     // self.size += mem::size_of_val(&size);
//     // self.print_cache_size();
    
//     // self.size += mem::size_of_val(&"expiry_".to_string());
//     // self.size += mem::size_of_val(&expirytime);

//     size
// }

    
    pub fn clear_cache(&self) {
        
        let now = Instant::now();

    let mut cstore=self.cstore.write().unwrap();
        let mut cache=cstore;
        // Use write method to get a lock guard
        cache.retain(|_, v| {
            
            let duration = Duration::from_secs(v.expirytime);
            
            let instant = now.checked_sub(duration).unwrap();
            
            now.duration_since(instant) < self.expiration
        });
    }
    // Add a method to print the total size of the cache
  pub fn print_cache_size(&self)
  ->i32
//   ->(u64,u64) 
  {
    
    let cstore=self.cstore.read().unwrap();

    // Use a single read lock guard to access the cache
    let cache = cstore;

    // Initialize a variable to store the total size
    let mut total_size = 0;
    let mut total_key_size = 0;
    let mut total_value_size = 0;

    // Initialize a mutable reference to the cache iterator
    let mut cache_iter = cache.iter();

    // let mut folsize=0;
    // Loop until the iterator returns None
    while let Some((key, value)) = cache_iter.next() {
        // Add the size of the key and the value to the total
        total_size += mem::size_of_val(key);
        total_key_size += mem::size_of_val(&value.expirytime);
        total_key_size += mem::size_of_val(&value.size);
        // if(!key.contains("expiry")){
        //     total_key_size+=mem::size_of_val(key);
        // }
        // total_size += mem::size_of_val(value);
        // if !key.starts_with("expiry_"){
        //     folsize+=value;

        // }

    }

    println!("cache:{}----numfolders:{}---onlynamesize:{}",sizeunit::size(total_size as u64,true),cache.len() as u64,sizeunit::size(total_key_size as u64,true));
    cache.len() as i32
    // Print the total size in bytes
    // println!("The cache size is {} bytes", total_size);
    // (total_size as u64,cache.len() as u64)
  }
  pub fn togglenosize(&self){
    let shouldsize;
    {

        shouldsize=!*self.nosize.read().unwrap();
    }
    let mut setsize=self.nosize.write().unwrap();
    *setsize=shouldsize;
    drop(setsize)
    // {
    //     println!("{:?}",*self.nosize.read().unwrap())
    // }
  }
  pub fn togglefolcount(&self){
    let tempstore;
    {

        tempstore=!*self.showfolderchildcount.read().unwrap();
    }
    let mut setstore=self.showfolderchildcount.write().unwrap();
    *setstore=tempstore;
    drop(setstore)
    // {
    //     println!("{:?}",*self.nosize.read().unwrap())
    // }
  }
  pub fn togglelsl(&self){
    let tempstore;
    {

        tempstore=!*self.loadsearchlist.read().unwrap();
    }
    let mut setstore=self.loadsearchlist.write().unwrap();
    *setstore=tempstore;
    drop(setstore)
    // {
    //     println!("{:?}",*self.nosize.read().unwrap())
    // }
  }
  pub fn foldercon(&self,path:&str)
  ->i32
//   ->(u64,u64) 
  {
    let cstore=self.cstore.write().unwrap();

    // Use a single read lock guard to access the cache
    let cache = cstore;

    // Initialize a variable to store the total size
    let mut total_size = 0;
    // let mut total_key_size = 0;
    // let mut total_value_size = 0;

    // Initialize a mutable reference to the cache iterator
    let mut cache_iter = cache.iter();

    // let mut folsize=0;
    // Loop until the iterator returns None
    while let Some((key, _)) = cache_iter.next() {
        if(key.starts_with(path))
        {
            total_size += 1;
        }
        // Add the size of the key and the value to the total
        
        // if(!key.contains("expiry")){
        //     total_key_size+=mem::size_of_val(key);
        // }
        // total_size += mem::size_of_val(value);
        // if !key.starts_with("expiry_"){
        //     folsize+=value;

        // }

    }

    // self.app_handle.emit_to(
    //     "main",
    //     "folder-size",
    //     {
    //       sizeunit::size(folsize,true)
    //     },
    //   )
    //   .map_err(|e| e.to_string()).unwrap();

    // println!("cache:{}----numfolders:{}---onlynamesize:{}",sizeunit::size(total_size as u64,true),cache.len() as u64,sizeunit::size(total_key_size as u64,true));
    total_size
    // Print the total size in bytes
    // println!("The cache size is {} bytes", total_size);
    // (total_size as u64,cache.len() as u64)
  }
  fn addthread(&self){
    let count = self.process_count.clone();
    *count.lock().unwrap() += 1; // increment the count every time the command is called
    // let result = count.clone();
  }
}
