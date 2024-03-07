import React from 'react';
// holy shit all of these are needed just for the drag and drop
import { DragDropContext, Draggable, Droppable,
OnDragEndResponder,
DraggableProvided, DroppableProvided } from "react-beautiful-dnd";

// NOTE TO SELF: this is the components of every app in the main page, so anything you
// add here will change every app
export interface ItemComponenets{
    id: string,
    components: JSX.Element,
}

interface DraggableListProps {
    droppableId: string;
    data: ItemComponenets[];
    onDragEnd: OnDragEndResponder;
    renderItem: (item: ItemComponenets, provided: DraggableProvided) => JSX.Element;
    renderWrapper: (children: JSX.Element, provided: DroppableProvided) => JSX.Element;
    direction?: 'vertical' | 'horizontal';
}

const DraggableList: React.FC<DraggableListProps> = ({
    droppableId,
    data,
    onDragEnd,
    renderItem,
    renderWrapper,
    direction = 'vertical',
   }) => (
    <DragDropContext onDragEnd={onDragEnd}>
       <Droppable droppableId={droppableId} direction={direction}>
         {(provided) =>
           renderWrapper(
             <>
               {data.map((item, index) => (
                 <Draggable key={item.id} draggableId={item.id} index={index}>
                   {(provided) => renderItem(item, provided)}
                 </Draggable>
               ))}
               {provided.placeholder}
             </>,
             provided
           )
         }
       </Droppable>
    </DragDropContext>
);

export default DraggableList;