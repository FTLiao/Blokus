//
//Blokus OOP專題  電物系:廖峰挺 謝勝雄
//
//
//4-26新增
//
initial.h 初始化						~/fred/Apr26.cpp  使用範例		

	void GenerateBLOCK(struct BLOCK)	生成方塊
	void InitialBOARD(struct BOARD)		初始化棋盤


//
//4-30新增
//		

menu.h 畫棋盤，預設格式(m,n) = (640,480)			~/fred/Apr30.cpp  使用範例

	void MENU_START(int x,int y)		傳入滑鼠位置，判斷是否在所選範圍內
						開始遊戲選項

	void MENU_HISTORY(int x,int y)		玩家記錄選項
						
	void MENU_TUTOR(int x,int y)		遊戲教學

	void MENU_QUIT(int x,int y)		遊戲結束
	
	void MENU()				draw初始遊戲使用者介面

	int *mouseclick()			回傳滑鼠動作
						x[0] = -1 Default
						       0  Left Click;
						       1  Right Click;
						x[1] = mousex();      x[2] = mousey();
//
//5-02修改
//

Boar.h 棋盤內容明確化
	struct PLAYER
	{
  		char         PlayerColor;
		struct BLOCK PlayerPiece[21];
	};
//
//5-05 增加方塊照片新增
//

//
//5-06
//
playgraph.h

	void PlayBackground()				繪畫遊戲背景
	void PrintPlayerBox(struct PLAYER A[])		遊戲玩家方塊選擇處
menu.h
	bool PLAY_TERMINATE(int,int,int)		遊戲是否終止
修改	int MENU_START()				設為遊戲開始後的main fun
修改	void MENU()					獨立選項highlight
新增	void MenuHighlight(int,int)			Highlight Menu頁的物件

main.cpp	串聯繪圖方程式

//
//5-07
//

BoardGraph.h			主要為繪畫棋盤上面事件

	void PlotBoard(struct BOARD)				判斷棋盤上的顏色，根據其繪上圖案
						主要是用readimagefile來完成
	void InitialPlotBoard()			為測試棋盤模樣的

建議可以試看看讓程式直接把棋盤顏色畫成一張圖，然後輸出成一個檔案，在讀回來程式裡面，
而不是一個一個點陣的讀，這樣或許可以減少讀取時間及runtime

//
//5-08
//
修改所有的block


新增
GraphData.h	void PlayerBoxData(char x[84][100])		主要儲存圖形資料
PlayGraph.h	
	新增void PrintMoveImage(char *name,int PieceIndex,int x,int y) 畫出移動時的方塊圖形

繪圖fun 新增 refresh 功能
	決定是否要再判別一次整個畫面，還是直接讀取舊的檔案即可。

//
//5-09
//
新增
@PlayGame.h
RePrint(struct PLAYER[] , struct BOARD[21][21] , int RefreshOfBox , int RefreshOfBoard)
考量到PrintMoveImage，需自行加上setvisualpage(page)，否則加在副程式裡的話，在move時圖會閃
@PlayGame.h (124~159) 
put 程式碼
畫圖很慢，會歪，建議將Block.txt重寫成置中樣式



修正	放方塊時，若不是在board上面，仍舊可以放下去的問題。		
	利用判斷board邊界外側兩個顏色及滑鼠位置，判定方塊是否在board內
	優點:設計、判斷簡易
	缺點:若設計AI時，此種方法應該不適用

//
//5-12
//
新增	MoveBoundaryCheck	判斷放下時方塊是否有在棋盤外
flip_turn.h
新增	void FlipTurnImage(struct BLOCK &b,int playerindex,int pieceindex,char path[50])
	傳遞Flipturn 後的圖形位置
修改	Move Put while迴圈
修改	PrintMoveImage內的判別式，讀取可圖檔

完成	紅色及黃色的圖形

//
//5-13 14
//

新增
Global.h	所有會使用到的變數及方程式

修改	
PlayGraph.h	void MoveCo(struct BLOCK &b) 判斷方塊角角
BoardGraph.h	int PutImageCheck()		判斷是否可以放置方塊


================================四人遊戲模式完成!!!!!!done!!=========================

//
//5-16
//

修改 playgame 判斷放下方塊的判斷式，由於if else if 會找最相近的，所以雖然是上一層的else if若沒有加上大括號，可能變成下一層if 的部分。

//
//5-19
//
修正	放方塊的function 改變旋轉方向，即可解決方塊放置錯誤的問題

新增 AI.h
進入偉大的航道 AI.idea 詳見

//
//5-23
//
 -lwinmm     for sound 

//
//5-31
//
Board update問題

//
//6-17
//

Debug 
1. setindex到七會放錯
2. 有時候AI不會印出來
3. AI會replace (0,0)
4. ESD沒作用? 0分
5. Fillscore會當掉
6. block 會被replace
7. playerbox變慢! book b[4][21]


解決圖形問題!!