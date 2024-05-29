# mcdonald_ordering

## **问题描述**

2023年5月，麦当劳在北邮开业。大量的学生去那里订餐。正因为如此，麦当劳的在线点餐系统经常关闭以避免拥挤，尤其是在午餐和晚餐时间。该系统的关闭时间不确定。北邮的学生认为这非常麻烦。

然而，北邮学生无所畏惧。北京邮电大学最优秀的学生之一(也是北邮ICPC团队的一员)Zhai Xie (ThomasX)在飞书上开发了一个实时监控机器人，它告诉我们麦当劳在线点餐系统的实时状态。有了这个机器人，北邮学生可以更方便地点餐。

在这个问题中，需要你像他一样完成这个任务，开发一个系统来模拟麦当劳的在线点餐系统。

北邮的麦当劳和它的点餐系统在![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps1.jpg)开始工作，在![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps2.jpg)关闭。麦当劳一共有![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps3.jpg)种食物和![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps4.jpg)种套餐类型，每种套餐中包含多种食物，具体配置信息将在菜单文件 (dict.dic) 中提供。对于制作和存储每种食物，规定第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps5.jpg)种食物在![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps6.jpg)秒内完成，其最大存储容量为![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps7.jpg) ，表示该种食物最多可以存储![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps8.jpg)个。麦当劳系统每天开放前，所有食物存储容量都为0，在任何时间点如果某种食物的存储量小于![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps9.jpg)，则会立即制作该食物，直到达到![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps10.jpg)。其中，不同种类食物可以同时制作，同种类食物只能依次制作。

从![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps11.jpg)到![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps12.jpg)(含)，学生可以在系统中点餐(如果系统未关闭)。每一天按照顺序有![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps13.jpg)个订单，第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps14.jpg)个订单发生在时间![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps15.jpg):![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps16.jpg):![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps17.jpg)，其要求一份![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps18.jpg)类型(![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps19.jpg)，其中![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps20.jpg)和![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps21.jpg)分别表示全体的套餐和食物的集合)的套餐或食物。如果点餐时系统关闭，会导致点餐失败。22:00以后如果还有之前的订单未完成，则麦当劳会继续加班，且保证23:59:59(含)前一定能完成所有订单。

对于订单处理存在如下规则：

• 在每一秒的开始，如果有新的食物完成，则首先存储食物，然后接受订单(如果存在)。

• 订单按照“先来先到，异步处理”原则进行处理。

– 先来先到：指的是对于有存量的食物，总会被分配给时间最早的订单(套餐或单点)。

– 异步处理：指的是当一个订单(套餐或单点)因为请求的食物没有被全部满足时，不必等待该订单完成，可以直接处理下一个订单。

• 食物一旦被分配给订单，就不能撤销。食物被分配给订单后，即便该订单尚未完成，该食物也不再占用对应类型的容量。

• 当订单(套餐或单点)中要求的所有食物，均已被分配给该订单，则该订单会立刻完成。

• 如果在某个时刻![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps22.jpg)，有人下了一个订单，并且该订单无法立刻完成，导致未完成订单的数量大于![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps23.jpg)，则系统立即自动关闭(不再接受订单)，但该订单仍然算作成功下单。

• 如果在某个时刻![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps24.jpg)，未完成订单的数量小于![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps25.jpg)，则系统将在1秒后重新打开。即系统可以接受![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps26.jpg)时刻的订单，而不能接受![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps27.jpg)时刻的订单。

你的系统需要输出：每一个订单是否下单成功，以及完成的时间。

## **菜单文件**

本题为大家提供麦当劳的菜单文件(dict.dic)，按如下格式给出：

第一行给出![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps28.jpg)和![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps29.jpg)，其中![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps30.jpg)表示食物的种类数，![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps31.jpg)表示套餐的种类数。

第二行包含![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps32.jpg)个字符串，每个字符串![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps33.jpg)表示第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps34.jpg)种食物的名称。

接下来![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps35.jpg)行，其中的第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps36.jpg)行包含多个字符串，第一个字符串![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps37.jpg)表示第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps38.jpg)个套餐的名称，后续的第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps39.jpg)个字符串![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps40.jpg)表示第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps41.jpg)个套餐中包含的第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps42.jpg)种食物的名称。

注：系统每次运行时所读取的菜单文件内容可能不一样。

## **输入**

第一行包含一个整数![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps43.jpg)(![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps44.jpg))表示订单个数。

第二行包含两个整数![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps45.jpg)(![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps46.jpg))。

第三行包含![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps47.jpg)个整数![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps48.jpg)(![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps49.jpg))，其中![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps50.jpg)表示第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps51.jpg)种食物的制作时长。

第四行包含![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps52.jpg)个整数![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps53.jpg)(![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps54.jpg))，其中![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps55.jpg)表示第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps56.jpg)种食物的最大存储容量。

对于接下来的![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps57.jpg)行，用格式类似于![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps58.jpg)的方式，给出第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps59.jpg)个订单的时间。然后输入一个字符串![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps60.jpg)，表示套餐或食物的名称(参见dict.dic)。所有订单时间一定在![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps61.jpg)内，同一个时间点不可能出现多个订单，第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps62.jpg)个订单一定早于第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps63.jpg)个(![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps64.jpg))，且保证23:59:59(含)前一定能完成所有订单。

具体参见input.txt

## **输出**

输出包括![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps65.jpg)行，按照订单顺序输出订单完成时间。对于第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps66.jpg)行，如果第![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps67.jpg)个订单不成功，则输出Fail；否则，输出这个订单完成的时间，时间格式与输入格式(![](file:///C:\Users\xw\AppData\Local\Temp\ksohtml17060\wps68.jpg))一致。

具体参见output.txt
