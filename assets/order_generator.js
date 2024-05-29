const foods = ["麦香鱼","双层吉士汉堡","麦辣鸡腿堡","辣板烧鸡腿麦满分","吉士蛋堡","香浓烘焙咖啡","中号薯条","玉米杯","可乐"]
const combos = ['经典套餐', '早餐套餐', '天天超值套餐']
const list = [...foods, ...combos]
const time_range = [7, 22]
const padStr = str => String(str).padStart(2, '0')
const orders = []

for (let i = time_range[0]; i < time_range[1]; i++) {
    let foods = l => Array.from({ 
        length: Math.ceil(l * Math.random() + 5) 
    }).map(() => list[Math.floor(list.length * Math.random())])
    let order = Array.from({ length: 10 })
        .map(e => Math.floor(Math.random() * 59))
        .sort((e1, e2) => e1 - e2)
        .map(e => `${padStr(i)}:${padStr(e)}:00 ${foods(Math.ceil(10 * Math.random())).join(' ')}`)
        .join('\r\n')
    orders.push(order)
}
console.log(orders.join('\r\n'))

