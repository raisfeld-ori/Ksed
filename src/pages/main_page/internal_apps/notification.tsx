import { useState } from 'react';
import './notification.css';
import success from '../assets/small-check-mark-icon.svg';
import failed from '../assets/cross-icon.svg';
import loading from '../assets/sand-clock-empty-icon.svg';

export enum NotificationType{
    Success, Failure, Loading, Other
}
function notification_icon(type: NotificationType){
    switch (type){
        case NotificationType.Failure: return failed
        case NotificationType.Success: return success
        case NotificationType.Loading: return loading
        case NotificationType.Other: return ''
    }
}
export interface NotificationData{
    type: NotificationType,
    name: string,
    text: string,
}

export default function NotificationSystem(){
    const [notifications, set_notifications] = useState<JSX.Element[]>([]);
    const new_notification = (data: NotificationData) => {
        let notification = <div className='notification' key={data.text}>
            <img className='image' src={notification_icon(data.type)}/>
            <h1 className='head'>{data.name}</h1>
            <p className='description'>{data.text}</p>
        </div>
        const done = setTimeout(() => {
            set_notifications(notifications.filter((other) => {return other != notification}));
            clearTimeout(done);
        }, 5000);
        set_notifications([...notifications, notification]);
    }
    let html = <div className="handler">
    {notifications}
    </div>

    return {html, new_notification};
}