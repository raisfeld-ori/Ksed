import App from '../App';
import { invoke } from '@tauri-apps/api';
import { useState, useEffect } from 'react';
import img from '../../assets/terminal.png';
import { open } from '@tauri-apps/api/dialog';
import folder from '../../assets/folder.png';
import alpha from '../../assets/daddyishome.png'
import './file_system.css';

async function upload_file(update_fs: () => Promise<void>, set_files: React.Dispatch<React.SetStateAction<React.JSX.Element[]>>){
    let name: string = await invoke('system_get', {key: 'name'});
    let password: string = await invoke('system_get', {key: 'password'});
    let file_selected = await open({});
    if (Array.isArray(file_selected)){
        for (let i = 0; i < file_selected.length;i++){
            //@ts-expect-error
            set_files(files => [...files, <File name={file_selected[i]} key={files.length + 1}/>])
        }
        await update_fs();
    }
    else if (file_selected == null){return;}
    else{
        make_file(name, password, file_selected, FileType.File);
        await update_fs();
    }
}


function File(props: {name: string, type: FileType}){


    return <div className='file'>
    <img src={props.type == FileType.File ? alpha: folder} className='file_img'/><br />
    <p className='file_name'>{props.name}</p>
</div>;
}

enum FileType{
    File, Directory,
}

async function make_file(name: string, password: string, file: string, type: FileType){
    switch (type){
        case FileType.Directory: {await invoke('mkdir', {name: file});break;}
        case FileType.File: {await invoke('mk', {name, password, fileName: file});break;}
    }
}

function file_system() : [JSX.Element, React.Dispatch<React.SetStateAction<string>>, JSX.Element, update_fs:  () => Promise<void>]{
    const [location, set_location] = useState("Home");
    const [files, set_files] = useState<React.JSX.Element[]>([]);
    const [{dx, dy}, set_positions] = useState({dx: 0, dy: 0});
    const [ctx_display, set_ctx_display] = useState('none');
    const update_fs = async () => {
        let files: [string, string][] = await invoke('ls', {});
        let pwd: string = await invoke('pwd', {});
        let files_divs = [];
        for (let i = 0;i < files.length;i++){
            let file_type = (files[i][1] == 'File') ? FileType.File : FileType.Directory;
            if (files[i][0].length > 10){
                files_divs.push(<File name={files[i][0].slice(0, 7) + '...'} key={i} type={file_type}/>);
            }
            else{
                files_divs.push(<File name={files[i][0]} key={i} type={file_type}/>);
            }
        }
        set_files(files_divs);
        set_location(pwd);
    }
    useEffect(() => {
        document.addEventListener("click", () => set_ctx_display('none'));
        return () => document.removeEventListener("click", () => set_ctx_display('none'));
    }, [])
    const right_click = (ev: React.MouseEvent) => {
        ev.preventDefault();
        set_ctx_display('inherit');
        set_positions({dx: ev.clientX, dy: ev.clientY});
    }
    const make_files = (file_type: FileType) => {
        const NewFile = () =>{
            const [editing, set_editing] = useState('inherit');
            const [text, set_text] = useState('');
            const done_editing = async (e: any) => {
                e.preventDefault();
                set_editing('none');
                let name: string = await invoke('system_get', {key: 'name'});
                let password: string = await invoke('system_get', {key: 'password'});
                await make_file(name, password, text, file_type);
                await update_fs();
            }
            
            return <div className='file' style={{display: editing}}>
            <img src={img} className='file_img'/><br />
            <input className='editing' onChange={e => set_text(e.target.value)}
            onBlur={done_editing} onKeyDownCapture={e => {if (e.key == 'Enter'){set_editing('none');}}} autoFocus></input>
        </div>
        }
        set_files([...files, <NewFile key={files.length + 1}/>]);
    }
    let context_menu = 
    <div className='ContextMenu'
    style={{
        top: dy + 2 + 'px',
        left: dx + 2 + 'px',
        display: `${ctx_display}`,
    }}>
    <button onClick={() => make_files(FileType.Directory)}>create dir</button>
    <br />
    <button onClick={async () => await upload_file(update_fs, set_files)}>upload file</button>    
    </div>;
    let Application = <div className='ApplicationDirectory'>
            <h1 className='filesystemtxt2'>{location}</h1>
        </div>
    let app_html = <div className='frametest2' onContextMenu={right_click}>
        {Application}
        <div className='file_grid'>
        <>{files}</>
        </div>
    </div>;
    const [display, set_display] = useState('none');
    let app = <App element={app_html} display={display} set_display={set_display} name='File System'/>;
    return [app, set_display, context_menu, update_fs];
};


export default file_system;