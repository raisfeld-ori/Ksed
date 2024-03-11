import App from './App';

function file_system(set_fs_html: React.Dispatch<React.SetStateAction<JSX.Element>>){
    return () => {
    let app_html = <h1>here is some code</h1>;
    let app = <App element={app_html} name='feet pics'/>;
    set_fs_html(app);
};
}

export default file_system;