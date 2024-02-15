import React, { useEffect, useState } from "react";
import { AlertDialog, AlertDialogContent, AlertDialogTitle, AlertDialogDescription, AlertDialogCancel, AlertDialogAction,AlertDialogHeader, AlertDialogFooter, AlertDialogTrigger } from "../components/ui/alertdialog";
import { operationfileinfo, setcolorpertheme } from "./greet";
import { Checkbox } from "../components/ui/checkbox";
import { invoke } from "@tauri-apps/api/tauri";
import { Input } from "../components/ui/input";
export default function NewLeaf({dest,isdir,showad,setshowad}){
    const [name,setname]=useState("")
    return (

      <AlertDialog open={showad} onOpenChange={setshowad}>
      
        <AlertDialogContent className={`${setcolorpertheme}`}>
          <AlertDialogHeader>
            <AlertDialogTitle>New {isdir?"Folder":"File"} name?</AlertDialogTitle>
            <AlertDialogDescription>
            <Input value={name}
                type="text"
                placeholder="Name"
                onChange={(event) =>
                  {
                    let pp=(event.target.value);
                    setname(pp)
                  }
                }/>
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction onClick={()=>{
                invoke("new",{
                    dest:dest,
                    isdir:isdir,
                    name:name
                  })
            }}>Continue</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    )
}