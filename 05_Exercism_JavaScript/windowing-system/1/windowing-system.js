// @ts-check

/**
 * Implement the classes etc. that are needed to solve the
 * exercise in this file. Do not forget to export the entities
 * you defined so they are available for the tests.
 */

export class Size{
  width = 80;
  height = 60;
  constructor(wh=80,ht=60){
    this.width = wh;
    this.height = ht;
  };
 resize(wg,ht){
    this.width = Math.max(1, wg);
    this.height = Math.max(1, ht);
  };
};

export class Position{
  x = 0;
  y = 0;
  constructor(px=0,py=0){
    this.x = px;
    this.y = py;
  }
  move(sx = 0,sy = 0){
    this.x = sx;
    this.y = sy;
  }
  
}

export class ProgramWindow {
  screenSize = new Size(800,600);
  size = new Size();
  position = new Position();

  resize(obj){
    const maxWidth = this.screenSize.width - this.position.x;
    const maxHeight = this.screenSize.height - this.position.y;

    this.size.resize(
      Math.min(obj.width, maxWidth),
      Math.min(obj.height, maxHeight)
    );
  }
  move(obj){
    const maxX = this.screenSize.width - this.size.width;
    const maxY = this.screenSize.height - this.size.height;

    this.position.move(
      Math.max(0, Math.min(obj.x, maxX)),
      Math.max(0, Math.min(obj.y, maxY))
    );
  }
  
}
export function changeWindow(obj){
    obj.size.width = 400;
    obj.size.height = 300;
    obj.position.x=100;
    obj.position.y = 150;
    return obj;
  };


