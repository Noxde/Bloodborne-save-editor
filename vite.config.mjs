import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { createHash } from "node:crypto";
import { readdirSync, readFileSync } from "node:fs";
import { resolve, relative } from "node:path";
function fingerprint() {
  const root=process.cwd(), hash=createHash('sha256');
  function add(path){for(const e of readdirSync(path,{withFileTypes:true}).sort((a,b)=>a.name.localeCompare(b.name,'en'))){const item=resolve(path,e.name);if(e.isDirectory())add(item);else if(e.isFile()){hash.update(relative(root,item).replaceAll('\\','/'));hash.update('\0');hash.update(readFileSync(item));}}}
  for(const path of ['src','public','src-tauri/src','src-tauri/resources']) add(resolve(root,path));
  for(const path of ['index.html','package.json','vite.config.mjs','src-tauri/tauri.conf.json','src-tauri/Cargo.toml']){hash.update(path);hash.update(readFileSync(resolve(root,path)));}
  return hash.digest('hex').slice(0,16);
}
export default defineConfig({ plugins: [react()], define: {__APP_VERSION__:JSON.stringify(fingerprint())}, clearScreen: false, server: {watch:{ignored:['**/src-tauri/**']}} });
