/// <reference path="./global.d.ts" />
// @ts-check

/**
 * Implement the functions needed to solve the exercise here.
 * Do not forget to export them so they are available for the
 * tests. Here an example of the syntax as reminder:
 *
 * export function yourFunction(...) {
 *   ...
 * }
 */
var recipe = {
  noodles: 200,
  sauce: 0.5,
  mozzarella: 1,
  meat: 100,
};

export function cookingStatus(time = -1){
  if(time==0) return 'Lasagna is done.';
  if(time == -1) return 'You forgot to set the timer.';
  return 'Not done, please wait.';
}

export function preparationTime(layers,min = 2){
  const len = layers.length;
  return len*min;
}

export function quantities(layers){
  const countNoodles = layers.filter(ele =>   ele==='noodles').length;
  const countSauce = layers.filter(ele =>   ele==='sauce').length;
  const obj = {'noodles' : countNoodles*50 , 'sauce' : countSauce*0.2};
  return obj;
}

export function addSecretIngredient(list1,list2){
  const lastEle = list1.pop();
  list1.push(lastEle);
  list2.push(lastEle);
}

export function scaleRecipe(obj = recipe,num=1){
  const scaled = {}
  for(const key in obj) {
    scaled[key] = (obj[key] * (num/2));
  }
  return scaled;
}
