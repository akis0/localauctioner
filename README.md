# localauctioner

HTTPリクエストで動かすようにしたい。Rocket使う。









users table (userid unique,name text,balance int)
items table (itemid unique,name text,startprice int,currentprice int,ownerid int,putupdate,deadlinedate,state)
bids table (bidid unique,itemid int,userid int,date,price int, success bool )
balancechange table (balancechangeid unique,payuserid int,getuserid, date,amount int)










telnetで接続して、ユーザーの追加、出品、入札、入札締め切り、出品物の閲覧などができるものを作りたい。
なりすまし、盗聴などセキュリティ関係は考慮していない

最初のrequestは何でも良い。
2回目に"adduser","bid","sell"finishbid,topup,exitの何れかを送信することで其々に対応した応答が返ってくるようにする。
adduserでデータベースに新しいユーザーを登録する。
sellでデータベースに出品物を登録する
bidで入札する
finishbidで入札を締め切る
topupで残高を増やす
exitで終了する。つもり。

sqlite3のデータベースの構造(予定) <br>
Users: userid(int,unique), username(text), balance(int) <br>
Items:itemid(int,unique), itemname(text),ownerid(int),startprice(int),expiration date<br>
Bids: bidid(int, unique), bidderid, itemid,bidprice(int) <br>
Finitems: itemid,itemname,ownerid,startprice,finishprice(int)<br>



名前被り対策はしてない。

送信する情報<br>
・ユーザー追加
"adduser"
ユーザー名
残高

・残高追加
"topup"
ユーザーid
追加額

・出品
"sell"
ユーザーid
商品名
開始額

・入札
"bid"
ユーザーid
item id
入札額

・オークション終了
"finish"


主な関数の簡単な説明(エラーハンドリングはまだ)
・handle_adduser
上記ユーザー追加の情報を受け取って、idをclientに送信する
・handle_sell
上記出品の情報を受け取ってitemsテーブルに登録してそのidをclientに送信する


(以下未実装)
・handle_finishbid
itemidで終了するものを指定して値段を確定させる
・handle_bid
上記入札の情報を受け取って、入札額が更新されたらbidテーブルに登録する。
・handle_topup
上記残高追加の情報を受け取って、残高を増減する。
・handle_showitems
出品数の一覧をclientに送信する
・inquiry_itemid
clientの出品物のidと入札額を送信する
