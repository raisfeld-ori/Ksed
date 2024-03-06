import './main_page.css';
import { DndProvider } from 'react-dnd';
import { HTML5Backend } from 'react-dnd-html5-backend';
import { useDrag } from 'react-dnd';
import ReactGridLayout from 'react-grid-layout';
import { useDrop } from 'react-dnd';

export interface DragItem {
    type: string;
    id: string;
}

interface DraggableItemProps {
 id: string;
}

let item_type = 'ITEM_TYPE_HERE';

const DraggableItem: React.FC<DraggableItemProps> = ({ id }) => {
 const [{ isDragging }, drag] = useDrag<DragItem, any, any>({
    type: item_type,
    item: { type: item_type, id },
    collect: (monitor) => ({
      isDragging: !!monitor.isDragging(),
    }),
 });

 return (
    <div ref={drag} style={{ opacity: isDragging ? 0.5 : 1 }}>
      {/* Your item content */}
    </div>
 );
};

interface DroppableAreaProps {
    onDrop: (item: DragItem) => void;
}

const DroppableArea: React.FC<DroppableAreaProps> = ({ onDrop }) => {
    const [, drop] = useDrop({
       accept: item_type,
       drop: (item: DragItem, monitor) => {
         onDrop(item);
       },
    });
   
    return <div ref={drop}>Drop Target</div>;
   };


const Grid: React.FC = () => {
 const layout = [
    { i: 'a', x: 0, y: 0, w: 1, h: 2 },
    { i: 'b', x: 1, y: 0, w: 3, h: 2 },
 ];

 const handleDrop = (id: DragItem) => {
    console.log(`Item ${id} dropped`);
};

 return (
    <ReactGridLayout className="layout" layout={layout} cols={12} rowHeight={30} width={1200}>
      <DraggableItem key="a" id="a" />
      <DraggableItem key="b" id="b" />
      <DroppableArea onDrop={handleDrop} />
    </ReactGridLayout>
 );
};



function App() {
 return (
    <div id='background'>
    <DndProvider backend={HTML5Backend}>
      <Grid />
    </DndProvider>
    </div>
 );
}

export default App;