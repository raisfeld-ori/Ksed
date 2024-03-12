import App from './App';
import { invoke } from '@tauri-apps/api';

function file_system(set_fs_html: React.Dispatch<React.SetStateAction<JSX.Element>>){
    return () => {
    let app_html = <div
     className='frametest2'>
        <p className='filesystemtxt'> --- Notes ----
        <br />✦ The world is full of justice. But there is no true justice.
        <br />✦ I don’t think it’s a crime to kill a villain. But I also don’t think it’s a good thing to kill someone who’s innocent.
        <br />✦ There are no heroes or villains. Just people who make decisions.
        <br />✦ You think you can just turn your back on the world and live in peace? You’re pathetic!
        <br />✦ Being a hero means fighting back even when it seems hopeless.
        <br />✦ The true power of an individual is to be able to master yourself.
        <br />✦ You can’t use society as an excuse to avoid facing yourself.
        <br />✦ If you really want to become strong, stop caring about what others think about you.
        <br />✦ I just punch until it stops moving. That’s it.
        <br />✦ I’ll keep pushing myself until I become the strongest.
        <br />✦ To become strong, you must first accept your own weaknesses.
        <br />✦ The true power of a hero lies not in their strength, but in their ability to inspire others.
        <br />✦ No matter how powerful you become, don’t become a monster in the process.
        <br />✦ I’m not saying I need fans, but it would be nice if someone noticed what I’ve been doing.
        <br />✦ Instead of sitting around frustrated it’s better to keep on moving forward.
        <br />✦ I will not grow if I cannot face superior opponents.
        <br />✦ Human strength lies in the ability to change yourself.
        <br />✦ Instead of sitting here frustrated, it’s better to keep moving forward.
        <br />✦ I’ll leave tomorrow’s problem to tomorrow’s me.
        </p>
     </div>;
    let app = <App element={app_html} name='Notes [DEV TEST]'/>;
    set_fs_html(app);
};
}

export default file_system;