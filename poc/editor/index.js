const uuid = require('uuid');
import * as gw from "mirovia_wasm";
import * as idb from 'idb-keyval';


let folders = [];
let explorer = {};
let explorer_items = {};
let current_file_id = null;


async function reload_folders() {
  idb.get('folders')
    .then((val) => {
      folders = val;
      refresh_explorer()
    });
}


async function add_folder() {
  let file_handle = await window.showDirectoryPicker();
  if (! await readwrite_permission(file_handle, true)) {
    console.error(`permission read/write not ok for ${file_handle.name}`);
  }
  folders.push(file_handle);
  idb.set('folders', folders)
    .then(() => console.log('folders set'))
    .catch((err) => console.error('error', err));
  refresh_explorer()
}


async function readwrite_permission(file_handle) {
  const opts = {
    'mode': 'readwrite'
  }
  if (await file_handle.queryPermission(opts) === 'granted') {
    return true;
  }
  if (await file_handle.requestPermission(opts) === 'granted') {
    return true;
  }
  return false;
}


async function refresh_explorer () {
  explorer = {};
  explorer_items = {};
  for (const folder of folders) {
     await refresh_explorer_folder(explorer, folder)
  }
  console.log(JSON.stringify(explorer, null, 2))
  let div_content = ''
  for (const [key, e] of Object.entries(explorer)) {
    div_content += `
      <div>
    `
    div_content = complete_explorer(div_content, e)
    div_content += `
      </div>
    `
  }
  folders_div.innerHTML = div_content
}


window.click_explorer_item = async (key) => {
  const handle = explorer_items[key].handle
  if (handle.kind !== "file") {
    return
  }
  current_file_id = key
  const fileData = await handle.getFile();
  const content = await fileData.text();
  explorer_items[key].content = content
  file_content_textarea.value = content
  file_title_span.innerHTML = explorer_items[key].name
  file_modified_span.innerHTML = ""
  file_title_span.classList.remove("modified");
  if (explorer_items[current_file_id].latest_value) {
    file_content_textarea.value = explorer_items[current_file_id].latest_value
    if (file_content_textarea.value !== content) {
      file_updated_force()
    }
  }
}


async function save () {
  const writable = await explorer_items[current_file_id].handle.createWritable()
  await writable.write(file_content_textarea.value)
  await writable.close()
  file_modified_span.innerHTML = ""
  file_title_span.classList.remove("modified");
}


function complete_explorer (div_content, e) {
  explorer_items[e.key] = e
  let slash = e.type === 'directory' ? '/' : ''
  div_content += `<span class="${e.type}" onclick="click_explorer_item('${e.key}')">${e.name}${slash}</span>`
  if (e.type === 'directory') {
    div_content += '<div class="childs_wrapper">'
    for (const [key, c] of Object.entries(e.childs)) {
      div_content += `
        <div class="child_wrapper">
      `
      div_content = complete_explorer(div_content, c)
      div_content += '</div>'
    }
    div_content += '</div>'
  }
  return div_content
}


async function refresh_explorer_folder(parent, folder) {
  const key = uuid()
  parent[key] = {
    'key': key,
    'type': 'directory',
    'name': folder.name,
    'childs': {},
    'handle': folder
  }
  await readwrite_permission(folder)
  const entries = folder.entries();
  while (true) {
    try {
      let entry = await entries.next();
      if (!entry.value) {
        break
      }
      const file_handle = entry.value[1];
      if (file_handle.kind === 'file') {
        await refresh_explorer_file(parent[key]['childs'], file_handle)
      } else if (file_handle.kind === 'directory') {
        await refresh_explorer_folder(parent[key]['childs'], file_handle)
      }
    } catch (error) {
      console.error(error)
      break
    }
  }
}


async function refresh_explorer_file(parent, file) {
  const key = uuid()
  parent[key] = {
    'key': key,
    'type': 'file',
    'name': file.name,
    'handle': file
  }
}


function go_fullscreen() {
  document.body.requestFullscreen();
}


function quit_fullscreen() {
  document.exitFullscreen();
}


function file_updated(e) {
  if(explorer_items[current_file_id].latest_value === file_content_textarea.value) {
    return
  }
  file_updated_force()
}


function file_updated_force() {
  file_modified_span.innerHTML = "○ "
  file_title_span.classList.add("modified");
  explorer_items[current_file_id].latest_value = file_content_textarea.value
}


function ctrl_s(e) {
    if((e.ctrlKey||e.metaKey) && e.which === 83){ // Check for the Ctrl key being pressed, and if the key = [S] (83)
        save()
        e.preventDefault();
        return false;
    }
    if((e.key=="Meta"||e.key=="Control")){ // Check for the Ctrl key being pressed, and if the key = [S] (83)
      e.preventDefault();
        return false;
    }
}


reload_folders_button.addEventListener('click', reload_folders)
add_folder_button.addEventListener('click', add_folder)
fullscreen_button.addEventListener('click', go_fullscreen)
quit_fullscreen_button.addEventListener('click', quit_fullscreen)
save_button.addEventListener('click', save)
file_content_textarea.addEventListener('keyup', file_updated)
document.addEventListener('keydown', ctrl_s);
document.addEventListener('keyup', ctrl_s);
