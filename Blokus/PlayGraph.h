#ifndef PlayGraph_h
#define PlayGraph_h 1
//除了PrintMoveImage之外，刷新畫面副程式
void RePrint(PLAYER A[4],int iPrintPlayerBox, int iPlotBoard){
	setactivepage(page=-page+1);
//	readimagefile("BKimage-board.bmp",0,0,m,n);
//    PlayBackground();
	PlotBoard(iPlotBoard);
	PrintPlayerBox(A,iPrintPlayerBox);
	
	settextstyle(4,0,2);
	setcolor(LIGHTBLUE);
	outtextxy(m-400,n-35,cscore[0]);	
	setcolor(YELLOW);
	outtextxy(m-325,n-35,cscore[1]);
	setcolor(LIGHTRED);
	outtextxy(m-250,n-35,cscore[2]);
	setcolor(LIGHTGREEN);
	outtextxy(m-175,n-35,cscore[3]);
	setcolor(LIGHTCYAN);
	for(int i=0;i<4;i++)
		if(A[i].PC_AI==-1)
			rectangle(m-405+75*i,n-35,m-365+75*i,n-15);
		
}
void PlayBackground()
{
  readimagefile("BoardUpdate.bmp",mbselect,0, m,n);
  setcolor(BLACK);
  settextstyle(10,0,3);
  setcolor(13);
  outtextxy(m-75,n-20,"Pause");
}

void InitialPlayerBox()
{
    char boxInit[4][40] ={"./BlocksPic/box-B.bmp",
                    "./BlocksPic/box-Y.bmp",
                    "./BlocksPic/box-R.bmp",
                    "./BlocksPic/box-G.bmp"};
    setactivepage(3);
    for(int i=0;i<4;i++)
    {
        readimagefile(boxInit[i],0,0,mbselect,n);
        writeimagefile(box[i],0,0,mbselect,n);
        if(i==playerindex)
            writeimagefile("PlayerBoxUpdate.bmp",0,0,mbselect,n) ;
    }
}

void PrintPlayerBox(struct PLAYER A[],int refresh)
{
    
    char Overlap[4][40]={"./BlocksPic/Overlap-B.gif",
    "./BlocksPic/Overlap-Y.gif",
    "./BlocksPic/Overlap-R.gif",
    "./BlocksPic/Overlap-G.gif"};
    char tmp[40];

  //切換圖片
  int j = 0;
  //歸一化畫面
  int mb = (int)(m*2.8/8/7/4);
  int nb = n/4/3/5;
  int shift = n/4;
  //

  if(refresh==1)//update全部的playerbox 
  {
    //setactivepage(3);    
    for(int k=0;k<4;k++)
    {
      readimagefile(box[k],0,0,mbselect,n);
      for(int i=0;i<4;i++)
      {
            if(i==k)
                strcpy(tmp,Overlap[i]);
            else
                strcpy(tmp,"./BlocksPic/Overlap-non.bmp");

            for(int j=0;j<21;j++)
            {
                if(A[i].PlayerPiece[j].USE ==false && PieceBuffer[i][j]==true)
                {
                    if(j<7)
                        {readimagefile(tmp,mb*(0 + 4*(j%7)) , shift*i+nb*0  ,mb*(0 + 4*(j%7)) +40 -1, shift*i+nb*0 +50 -2);}
                    else if(j<14)
                        {readimagefile(tmp,mb*(0 + 4*(j%7)) , shift*i+nb*5  ,mb*(0 + 4*(j%7)) +40 -1, shift*i+nb*5 +50 -1);}
                    else if(j<21)
                        {readimagefile(tmp,mb*(0 + 4*(j%7)) , shift*i+nb*10  ,mb*(0 + 4*(j%7)) +40 -1, shift*i+nb*10 +50 -3);}
                    
                }
            }
        
      }
      writeimagefile(box[k],0,0,mbselect,n) ;
    }
      //setactivepage(page);
      readimagefile(box[playerindex],0,0,mbselect,n);   
//      _sleep(200);
      writeimagefile("PlayerBoxUpdate.bmp",0,0,mbselect,n) ;   

  }
  else if(refresh ==2)//只update當前在使用的playerbox  
  {
    //setactivepage(2);
    readimagefile(box[playerindex],0,0,mbselect,n);
            strcpy(tmp,Overlap[playerindex]);

            for(int j=0;j<21;j++)
            {
                if(A[playerindex].PlayerPiece[j].USE ==false && PieceBuffer[playerindex][j]==true)
                {
                    if(j<7)
                        {readimagefile(tmp,mb*(0 + 4*(j%7)) ,shift*playerindex+ nb*0  ,mb*(0 + 4*(j%7)) +40 -1,shift*playerindex+ nb*0 +50 -2);}
                    else if(j<14)
                        {readimagefile(tmp,mb*(0 + 4*(j%7)) ,shift*playerindex+ nb*5  ,mb*(0 + 4*(j%7)) +40 -1,shift*playerindex+ nb*5 +50 -1);}
                    else if(j<21)
                        {readimagefile(tmp,mb*(0 + 4*(j%7)) ,shift*playerindex+ nb*10  ,mb*(0 + 4*(j%7)) +40 -1,shift*playerindex+ nb*10 +50 -3);}
                    
                }
            }
      writeimagefile("PlayerBoxUpdate.bmp",0,0,mbselect,n) ;
    //setactivepage(page);
    readimagefile("PlayerBoxUpdate.bmp",0,0,mbselect,n);

  }
  else if(refresh == 3)//把當前要印的box 複製到update上 
  {
    //setactivepage(page);    
    readimagefile(box[playerindex],0,0,mbselect,n);
    writeimagefile("PlayerBoxUpdate.bmp",0,0,mbselect,n);
  }
  else//沒有要幹麻時只印update 
  {
    //setactivepage(page);    
    readimagefile("PlayerBoxUpdate.bmp",0,0,mbselect,n);
  }
  
}



void MoveCo(struct BLOCK &b)
{
    for(int i=0;i<4;i++)
	   movexy[i]=3;
	for(int i=0;i<7;i++)
	   for(int j=0;j<7;j++)
	   {
            if(b.X[i][j]=='D'||b.X[i][j]=='d')
            {
                if(j<movexy[0])
                    movexy[0]=j;    //記錄最左邊 
                if(i<movexy[1])     //紀錄最上面 
                    movexy[1]=i;
                if(j>movexy[2])     //紀錄最右邊 
                    movexy[2]=j;
                if(i>movexy[3])     //紀錄最下面 
                    movexy[3]=i;
            }
        }
    for(int i=0;i<4;i++)
        movexy[i]-=3;
}

void PrintMoveImage(char *name,int PieceIndex,int x,int y,int turn)
{
    readimagefile(name,x+movexy[0]*s-s/2,y+movexy[1]*s-s/2,x+movexy[2]*s+s/2,y+movexy[3]*s+s/2);
}

void BoxHighlight(int x,int y)
{

  int mb = (int)(m*2.8/8/7/4);
  int nb = n/4/3/5;
  mb *=4;
  nb *=5;
  if((x<m && x>m-75)&&(y<n && y>n-20))
  {
    setcolor(15);rectangle(m-75,n-20,m-1,n-1);
    }
  else
  {
    setcolor(BLACK);rectangle(m-75,n-20,m-1,n-1);
    }

  //Hightlight for current PLayerBox
  for(int i=0;i<7;i++)
    for(int j=3*playerindex;j<3*playerindex+3;j++)
  		if( (x>mb*(i) && x < mb*(i+1)) && (y >nb*j && y < nb*(j+1)))
        {    
    		setcolor(BLACK);
            rectangle(mb*i-1,nb*j-1,mb*(i+1)-1,nb*(j+1)-1-1);
        }
}

void QuitHighlight()
{
    char quitblock[4][80] ={"./BlocksPic/Command/returnbox-B.gif","./BlocksPic/Command/returnbox-Y.gif",
                            "./BlocksPic/Command/returnbox-R.gif","./BlocksPic/Command/returnbox-G.gif"};
    if(mousex() > 0 && mousex()<mbselect && mousey()> n/4*playerindex && mousey()< n/4*(playerindex+1) )
        readimagefile(quitblock[playerindex],0,n/4*playerindex-2,mbselect,n/4*(playerindex+1)-1)  ;                 
                    
}

int QuitBlockCheck()
{
    if(mousex() > 0 && mousex()<mbselect && mousey()> n/4*playerindex && mousey()< n/4*(playerindex+1) )
        return 1;
    else 
        return 0;
}
void blockhighlight(const int &iput,const int &jput)
{
    readimagefile("./BlocksPic/Command/effect.gif",mbselect+mbside+(jput-2)*s-s/2,nbside+(iput-2)*s-s/2,mbselect+mbside+(jput+2)*s+s/2,nbside+(iput+2)*s+s/2);
//    _sleep(500);
}

void PauseMenu()
{
    setactivepage(page=-page+1);
    readimagefile("./BlocksPic/Command/cover.gif",0,0,800,614);
    setvisualpage(page);
}

#endif
