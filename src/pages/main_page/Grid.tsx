import React, { useEffect } from "react";
import { AppInterface } from "./internal_apps/App";

function Grid(props: {apps: ((dx: number, dy: number, draggableRef: (nodeEle: any) => void) => JSX.Element)[], 
  gridSize: number, margin: number}) {
  let apps = [];
  for (let i = 0, dxy = 0; i < props.apps.length; i++, dxy += props.margin) {
    let [ref, dx, dy] = useDraggable({gridSize: props.gridSize});
    // there's a couple errors since dx and dy are states and numbers at the same time
    // @ts-expect-error
    dy += dxy;
    // @ts-expect-error
    let app = props.apps[i](dx, dy, ref);
    
    apps.push(app);
  }
  return <div className="grid"><>{apps}</></div>
}
export default Grid;

export function desktop_app(name: string, image: string, app: AppInterface, visible: boolean){
    useEffect(() => {app.set_display('none');}, [])
  return function MakeApp(dx: number, dy: number, ref: ((nodeEle: any) => void)){
  return <div   key={name}>
    {app.context_menu}
    {app.screen}
    {visible ?
    <div className="draggable"
  ref={ref}
  style={{
  transform: `translate3d(${dx}px, ${dy}px, 0)`,
    }}
    onDoubleClick={() => {app.update();app.set_display('inherit');}}>
      <img src={image} alt={name} className="icon" />
      <p className="name">{name}</p>
  </div> : <div></div>
  }
    </div>}
}

export const useDraggable = ({gridSize} : {gridSize : number}) => {
  const [node, setNode] = React.useState<HTMLElement | null>(null);
  const [{ dx, dy }, setOffset] = React.useState({
      dx: 0,
      dy: 0,
  });

  const ref = React.useCallback((nodeEle: any) => {
      setNode(nodeEle);
  }, []);

  const handleMouseDown = React.useCallback((e: MouseEvent) => {
      const startPos = {
          x: e.clientX - dx,
          y: e.clientY - dy,
      };

      const handleMouseMove: any = (e: React.MouseEvent) => {
          const dx = e.clientX - startPos.x;
          const dy = e.clientY - startPos.y;
          const snappedX = Math.round(dx / gridSize) * gridSize;
          const snappedY = Math.round(dy / gridSize) * gridSize;
          setOffset({ dx: snappedX, dy: snappedY });
          updateCursor();
      };

      const handleMouseUp = () => {
          document.removeEventListener('mousemove', handleMouseMove);
          document.removeEventListener('mouseup', handleMouseUp);
          resetCursor();
      };
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
  }, [dx, dy]);

  const handleTouchStart = React.useCallback((e: TouchEvent) => {
      const touch = e.touches[0];

      const startPos = {
          x: touch.clientX - dx,
          y: touch.clientY - dy,
      };

      const handleTouchMove = (e: TouchEvent) => {
          const touch = e.touches[0];
          const dx = touch.clientX - startPos.x;
          const dy = touch.clientY - startPos.y;
          const snappedX = Math.round(dx / gridSize) * gridSize;
          const snappedY = Math.round(dy / gridSize) * gridSize;
          setOffset({ dx: snappedX, dy: snappedY });
          updateCursor();
      };

      const handleTouchEnd = () => {
          document.removeEventListener('touchmove', handleTouchMove);
          document.removeEventListener('touchend', handleTouchEnd);
          resetCursor();
      };

      document.addEventListener('touchmove', handleTouchMove);
      document.addEventListener('touchend', handleTouchEnd);
  }, [dx, dy]);

  const updateCursor = () => {
      document.body.style.userSelect = 'none';
  };

  const resetCursor = () => {
      document.body.style.removeProperty('user-select');
  };
  React.useEffect(() => {
      if (!node) {
          return;
      }
      node.addEventListener("mousedown", handleMouseDown);
      node.addEventListener("touchstart", handleTouchStart);
      return () => {
          node.removeEventListener("mousedown", handleMouseDown);
          node.removeEventListener("touchstart", handleTouchStart);
      };
  }, [node, dx, dy]);

  return [ref, dx, dy];
};