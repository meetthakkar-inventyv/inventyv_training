const fun1 = function (arr){
  let first_ele = arr.shift();
  fun2(first_ele,arr);
}
function PromiseFunction(arr) {
  PromiseToReturn = new Promise((resolve,reject) => {
    const sum = arr.reduce((accumulator, currentValue) => accumulator + currentValue, 0);
    if(sum > 35){
      resolve(`Promise is resolved with sum ${sum}`);
    }
    else{
      reject(`Promise is rejected with sum ${sum}`);
    }
  })
  return PromiseToReturn;
}
const fun2 = function(first_ele,arr){
  let arr2 = [3,3,2,2,1];
  arr2.unshift(first_ele);
  arr2 = [...arr2, ...arr];
  // console.log(arr2);
  PromiseFunction(arr2)
    .then(message => console.log(message))
    .catch(error => console.log(error));
}

fun1([1,2,1,2,1]);