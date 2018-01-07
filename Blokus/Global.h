#ifndef Global_h
#define Global 1
#define m 800
#define n 600
int page = 0;
int movexy[4];
int mbside = ((int)(m/8*5.2) - 21 * (int)(m/8*5.2/21))/2;
int nbside = (n - 21*(int)(m/8*5.2/21))/2;
char CharOnBoard[]={'B','Y','R','G'};
//玩家棋子選擇區寬
int mbselect = (int)(m/8*2.8);
int s = (int)(m/8*5.2/21);
int cnr[2][2]={ { mbselect+mbside , mbselect+mbside+21*s },
		  	    {          nbside ,          nbside+21*s } };
int mouse[3];
int mousebuffer[3]={0};
int playerindex;
int pieceindex;
int lastpiece[4];
int setindex;
int pos[4][2] = {{1,0},{0,1},{-1,0},{0,-1}};
bool PieceBuffer[4][21];

int score[4]={0};       //計算玩家分數 
char cscore[4][100];    //

struct list
{
	int X;
	int Y;
	list *next;
};

list *Eonboard[4] = {NULL};

struct BLOCK
{
  char X[7][7];     // 'D' 顏色  'E' 角落  'S' 邊界 
  int  TURN;        //0~3
  int  FLIP;        //0,1
  int  NO;          //個數 
  bool USE;         //代表可否使用? true可用
  struct list *Dlistp;	//D的位置 
  struct list *Elistp;	//edge的位置  
};

struct PLAYER
{
  char         PlayerColor;
  struct BLOCK PlayerPiece[21];
  int          PC_AI;       //0是player 1是AI  -1代表遊戲終止 
}A[4];  

struct BOARD 
{
  char WHO;         //1. 存格子放的玩家顏色         先判斷此是誰的方塊，在判斷color 
                    //判定那一個格子是否有值?       有值一定不可以放東西! 
                        
  char BOARDCOLOR[4];   //四個顏色 
                        //格子放哪個顏色的邊 紀錄s 
                        //格子放哪個顏色的角 紀錄e
                        //空白位置           紀錄#
  int  EDGESCORE[4];    //由此角延伸出去的block分數 

}Board[21][21],BufferBoard[21][21];

//Initial.h
void GenerateBLOCK(struct BLOCK []);
void InitialBOARD();
void resetmouse();
void BoardToBuffer();
void PieceToBuffer();
//menu.h
void menupage();
bool PLAY_TERMINATE();
void MENU_SETTING(int,int);
void MENU_TUTOR(int,int);
void MENU_QUIT(int,int);
void MENU();
void mouseclick();
void MenuHighlight(int,int);
void SettingHighlight();
//PlayGame.h
int SelectMode();
void InitialMode1();
int PlayerSelect();
void InitialSetting();
int MENU_START();
void PlayerPut();
//PlayGraph.h
void PlayBackground();
void InitialPlayerBox();
void PrintPlayerBox(struct PLAYER [],int );
void RePrint(struct PLAYER [],int,int);
void MoveCo(struct BLOCK &);
void PrintMoveImage(char *,int ,int,int,int);
void BoxHighlight(int ,int);
void QuitHighlight();
void blockhighlight(const int &,const int &);
int QuitBlockCheck();
//BoardGraph.h
int PlotBoard(int);
int MoveBoundaryCheck(const int &,const int &);
int PutImageCheck(const int &,const int &);
void HighLightEdge(int);
//AI
int edgex=-1,edgey=-1;    //掃到edge的點 
void Period_B();
void FindEage();
void On_End(int,int);
int RandPut();
void GameOver();
//Score.h
void CalScore();
void ShowScore();
void GameReset();
//Flip_Turn
void Flip(struct BLOCK &);
void Turn(struct BLOCK &);
void FlipTurnImage(struct BLOCK &,int ,int,char []);
void CallString(int,char [],char []);


int musiccount=0;
char musicpath[20]="./../music-1.wav";
char path[50];//for flip turn


char box[4][40] ={"./BlocksPic/box-Bupdate.bmp",
                    "./BlocksPic/box-Yupdate.bmp",
                    "./BlocksPic/box-Rupdate.bmp",
                    "./BlocksPic/box-Gupdate.bmp"};

char boxbuffer[4][40] ={"./BlocksPic/box-Bbuffer.bmp",
                    "./BlocksPic/box-Ybuffer.bmp",
                    "./BlocksPic/box-Rbuffer.bmp",
                    "./BlocksPic/box-Gbuffer.bmp"};

void boardtest()
{
    /*
    for(int k=0;k<4;k++){
    for(int i=0;i<21;i++){
	       for(int j=0;j<21;j++)
	           printf("%2c",Board[i][j].BOARDCOLOR[k]);
	        printf("\n");}
	        printf("\n--------------%d------------------\n",k);}*/
    for(int i=0;i<21;i++){
	       for(int j=0;j<21;j++)
	           printf("%2c",Board[i][j].WHO);
	        printf("\n");}
	        printf("------------------bufferboard---------------\n");
    for(int i=0;i<21;i++){
	       for(int j=0;j<21;j++)
	           printf("%2c",BufferBoard[i][j].WHO);
	        printf("\n");}
	        printf("\n\n");
}
void blocktest(const struct BLOCK &B)
{
    for(int i=0;i<7;i++){
        for(int j=0;j<7;j++)
//            if(B.X[j][i]!='#')
                printf("%c ",B.X[i][j]);
         printf("\n");
    }
}
void piecetest()
{
    printf("%d player ->\n",playerindex);
    for(int i=0;i<21;i++)
    {
        if(A[playerindex].PlayerPiece[i].USE)
            printf(" %2d Ye",i);
        else
            printf(" %2d No",i);
    }
    printf("\n");
    printf("%d Buffer ->\n",playerindex);
    for(int i=0;i<21;i++)
    {
        if(PieceBuffer[playerindex][i])
            printf(" %2d Ye",i);
        else
            printf(" %2d No",i);
    }
    printf("\n");
        
}


#endif
