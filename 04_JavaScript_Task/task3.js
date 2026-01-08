// const toSymbol  = (num) => Symbol(num);
const fromSymbol = (sym) => Number(sym.description);
let symArr = [Symbol(1),Symbol(2),Symbol(3),Symbol(4),Symbol(5)];
// console.log(myArr);
// let symArr = myArr.map(a=>toSymbol(a));
// console.log(symArr);

const PromiseFunction = (symArr) => {
  return new Promise((resolve,reject) => {
    let sum = symArr.reduce((accumulator, currentValue) => accumulator + fromSymbol(currentValue), 0);
    if(sum > 35){
      resolve(`Promise is resolved with sum ${sum}`);
    }
    else{
      reject(`Promise is rejected with sum ${sum}`);
    }
  })
}

const fun2 = (first_ele,symArr) => {
  let symArr2 = [Symbol(6),Symbol(7),Symbol(8),Symbol(9),Symbol(10)];
  // let symArr2 = myArr2.map(a=>toSymbol(a));
  symArr2.unshift(first_ele);
  symArr2 = [...symArr2, ...symArr];
  PromiseFunction(symArr2)
    .then(message => console.log(message))
    .catch(error => console.log(error));
}
const fun1 = (symArr) => {
  let first_ele = symArr.shift();
  // console.log(symArr);
  fun2(first_ele,symArr);
}
fun1(symArr);