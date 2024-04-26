import App, {AppInterface} from '../../App';
import { invoke } from '@tauri-apps/api';
import { dialog } from '@tauri-apps/api';
import { useState, useEffect, Dispatch } from 'react';
import { open, save } from '@tauri-apps/api/dialog';
import folder from '../../../assets/folder.png';
import alpha from '../../../assets/image-solid.svg';
import text from '../../../assets/pencil-square-icon.svg';
import arrowleft from '../../../assets/arrowleft.png'
import './file_system.css';

async function upload_file(update_fs: () => Promise<void>){
    let name: string = await invoke('system_get', {key: 'name'});
    let password: string = await invoke('system_get', {key: 'password'});
    let file_selected = await open({});
    if (Array.isArray(file_selected)){
        for (let i = 0; i < file_selected.length;i++){
            await invoke('upload_file', {name, password, file_path: file_selected});
        }
        await update_fs();
    }
    else if (file_selected == null){return;}
    else{
        // i should make a window for when upload_file fails, but it's not top priority for now
        await invoke('upload_file', {name, password, filePath: file_selected});
        await update_fs();
    }
}

async function export_file(file: string, update_fs: () => Promise<void>){
    let name: string = await invoke('system_get', {key: 'name'});
    let password: string = await invoke('system_get', {key: 'password'});
    let location = await save();
    if (location == null){return;}
    else{
        console.log(location);
        // same as in the function upload_file, i need to make a failsafe window, but it's not top priority
        await invoke('export_file', {name, password, file, location});
        await update_fs();
    }

}

function File(props: {name: string, type: FileType, update_fs: () => Promise<void>, 
    is_selected: Dispatch<string>, open_file: (file: string) => Promise<void>}){
    async function open(){
        if (props.type == FileType.Directory){
            await invoke('cd', {new: props.name});
            await props.update_fs();
        }
        else{
            props.open_file(props.name);
        }
    }
    const [file_extension, setFileExtension] = useState('');

    useEffect(() => {
        const fetchFileExtension = async () => {
            const extension: string = await invoke('gather_type', {file: props.name});
            setFileExtension(FileExtension(extension));
        };

        fetchFileExtension();
    }, [props.name]);
    if (props.name.length > 10){
        return <div className='file' onDoubleClick={open}
        onContextMenu={() => props.is_selected(props.name)}>
        <img src={props.type == FileType.File ? file_extension: folder} className='file_img'/><br />
        <p className='file_name'>{props.name.slice(0, 7) + '...'}</p>
        </div>;
    }
    else{
    return <div className='file' onDoubleClick={open} 
    onContextMenu={() => props.is_selected(props.name)}>
    <img src={props.type == FileType.File ? file_extension: folder} className='file_img'/><br />
    <p className='file_name'>{props.name}</p>
    </div>;
    }
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

export const FileExtension = (val: string) => {
    switch (val){
        case "Video": return ""
        case "Image": return alpha
        case "Text": return text
        case "HTML": return ""
        case "Unknown": return ""
        case "Audio": return ""
        default: return ""
    }
}

function file_system(open_file: (file: string) => Promise<void>) : AppInterface{
    const [location, set_location] = useState("Home");
    const [selected, set_selected] = useState('');
    const [files, set_files] = useState<React.JSX.Element[]>([]);
    const [{dx, dy}, set_positions] = useState({dx: 0, dy: 0});
    const [ctx_display, set_ctx_display] = useState('none');
    const update = async () => {
        let files: [string, string][] = await invoke('ls', {});
        let pwd: string = await invoke('pwd', {});
        let files_divs = [];
        for (let i = 0;i < files.length;i++){
            let file_type = (files[i][1] == 'File') ? FileType.File : FileType.Directory;
            files_divs.push(<File name={files[i][0]} key={i} type={file_type} 
                update_fs={update} is_selected={set_selected}open_file={open_file}/>);
        }
        set_files(files_divs);
        set_location(pwd);
    }
    useEffect(() => {
        let click = () => {set_ctx_display('none');set_selected('');};
        document.addEventListener("click", click);
        return () => document.removeEventListener("click", click);
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
            let [icon, set_icon] = useState(file_type == FileType.Directory ? folder : alpha);
            const done_editing = async (e: any) => {
                e.preventDefault();
                set_editing('none');
                if (text == ''){return;}
                if (await invoke('file_exists', {fileName: text})){
                    let sure = await dialog.confirm('a file with this name already exists, are you sure you want to delete it?');
                    if (sure){await invoke('rm', {file: text});}
                    else {return;}
                }
                let name: string = await invoke('system_get', {key: 'name'});
                let password: string = await invoke('system_get', {key: 'password'});
                await make_file(name, password, text, file_type);
                await update();
            }
            const set_new_icon = async (text: string) => {
                if (file_type == FileType.Directory) {return;}
                let new_icon: string = await invoke('gather_type', {file: text});
                set_icon(FileExtension(new_icon));
            }
            return <div className='file' style={{display: editing}}>
            <img src={icon} className='file_img2'/><br />
            <input className='editing' onBlur={done_editing} 
            onChange={async e => {set_text(e.target.value);set_new_icon(e.target.value);}}
            onKeyDownCapture={e => {if (e.key == 'Enter'){set_editing('none');}}} autoFocus></input>
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

    <button className='buttoncontextmenu' onClick={() => make_files(FileType.Directory)}>Create Folder</button>
    <button className='buttoncontextmenu' onClick={() => make_files(FileType.File)}>create file</button>
    <button className='buttoncontextmenu' 
    onClick={async () => await upload_file(update)}>Upload File</button>
    {selected != '' ?
        <div>
        {/*<button className='buttoncontextmenu' >Rename</button>*/}
        <p className='linecontextmenu'></p>
        <button className='buttoncontextmenu' onClick={async () => {await invoke('rm', {file: selected});await update();}}>
            Delete
        </button>
        <button className='buttoncontextmenu' onClick={async () => {await export_file(selected, update)}}>
            Export
        </button>
        </div>
        : <div></div>}
    </div>;
    let Application = <div className='ApplicationDirectory'>
            <button onClick={async () => {await invoke('cd_back', {});await update();}}>
                <img className='backdir' src={arrowleft} alt="arrowleft" />
            </button>
            <h1 className='filesystemtxt2'>{location}</h1>
        </div>
    let app_html = <div className='frametest2' onContextMenu={right_click}>
        {Application}
        <div className='file_grid'>
        <>{files}</>
        </div>
    </div>;
    let [screen, set_display, fullscreen] = App(app_html, 'file system');
    return {screen, context_menu, update, set_display, fullscreen};
};


export default file_system;