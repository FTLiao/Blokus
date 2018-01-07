#ifndef BoardGraph_h
#define BoardGraph_h 1

int PlotBoard(int refresh)
{
  //如果不需要判斷顏色則refresh = 0
  //放下方塊要判斷顏色則refresh = 1
  if(refresh)
  {
//    setactivepage(2);
    readimagefile("BoardUpdate.bmp",mbselect,0, m,n);
    boardtest();  
    for(int i=0;i<21;i++)
      for(int j=0;j<21;j++)
      {
//        if((Board[i][j].WHO != '#') && (BufferBoard[i][j].WHO == '#'))
//        {
            if((Board[i][j].WHO == 'B') && (BufferBoard[i][j].WHO == '#'))
              readimagefile("./BlocksPic/Board/blue.gif",  mbselect + mbside +s*j,nbside +s*i,mbselect + mbside+s*(j+1),nbside+s*(i+1));
            else if((Board[i][j].WHO == 'Y')&& (BufferBoard[i][j].WHO == '#'))
              readimagefile("./BlocksPic/Board/yellow.gif",mbselect + mbside +s*j,nbside +s*i,mbselect + mbside+s*(j+1),nbside+s*(i+1));
            else if((Board[i][j].WHO == 'R')&& (BufferBoard[i][j].WHO == '#'))
              readimagefile("./BlocksPic/Board/red.gif",   mbselect + mbside +s*j,nbside +s*i,mbselect + mbside+s*(j+1),nbside+s*(i+1));
            else if((Board[i][j].WHO == 'G')&& (BufferBoard[i][j].WHO == '#'))
              readimagefile("./BlocksPic/Board/green.gif", mbselect + mbside +s*j,nbside +s*i,mbselect + mbside+s*(j+1),nbside+s*(i+1));
//        }
      }
    writeimagefile("BoardUpdate.bmp",mbselect,0, m,n);  
//    _sleep(200);
//    system("pause");
//    setactivepage(page);
    readimagefile("BoardUpdate.bmp",mbselect,0, m,n);
  }
  else
  {
    readimagefile("BoardUpdate.bmp",mbselect,0, m,n);
  }
  return 1;
}

int MoveBoundaryCheck(const int &iput,const int &jput)
{

    int x,y;
    for(int i=0;i<7;i++)
       for(int j=0;j<7;j++)
           if(toupper(A[playerindex].PlayerPiece[pieceindex].X[i][j]) == 'D')
           {
                x = iput+i-3;
                y = jput+j-3;
                if(x>20 || y>20 || x<0 || y<0)
                {
                    return 0;
				}
            }
    return 1;
}

int PutImageCheck(const int &iput,const int &jput)
{
	int x,y,OnEagexy=0;
	
	for(int i=0;i<7;i++)
		for(int j=0;j<7;j++)
		{
			x = iput+i-3;
            y = jput+j-3;

	        if(toupper(A[playerindex].PlayerPiece[pieceindex].X[i][j]) == 'D')
	        {
				if(Board[x][y].WHO != '#')
					return 0;
                else if(Board[x][y].BOARDCOLOR[playerindex] == 's')
					return 0;
            }
        }
    for(int i=0;i<7;i++)
       for(int j=0;j<7;j++)
	   {
            x = iput+i-3;
            y = jput+j-3;
		    if( A[playerindex].PlayerPiece[pieceindex].X[i][j] == 'D'&& Board[x][y].BOARDCOLOR[playerindex] == 'e')
		    {
				OnEagexy = 1;
    				if(x==edgex && y==edgey)
	       				return 2;

			}
		}
	return OnEagexy;
}

void HighLightEdge(int playerindex)
{
    if(playerindex == 0)
        setcolor(BLUE);
    else if(playerindex == 1)
        setcolor(BROWN);
    else if(playerindex == 2)
        setcolor(RED);
    else
        setcolor(GREEN);
    for(int i=0;i<21;i++)
        for(int j=0;j<21;j++)
            if(Board[i][j].WHO == '#')
              if(Board[i][j].BOARDCOLOR[playerindex] == 'e')
              {
                if(playerindex==0)readimagefile("./BlocksPic/Command/edge-B.gif",mbselect+mbside+s*j,nbside+s*i,mbselect+mbside+s*(j+1),nbside+s*(i+1)+1);
                else if(playerindex==1)readimagefile("./BlocksPic/Command/edge-Y.gif",mbselect+mbside+s*j,nbside+s*i,mbselect+mbside+s*(j+1),nbside+s*(i+1)+1);
                else if(playerindex==2)readimagefile("./BlocksPic/Command/edge-R.gif",mbselect+mbside+s*j,nbside+s*i,mbselect+mbside+s*(j+1),nbside+s*(i+1)+1);
                else if(playerindex==3)readimagefile("./BlocksPic/Command/edge-G.gif",mbselect+mbside+s*j,nbside+s*i,mbselect+mbside+s*(j+1),nbside+s*(i+1)+1);
            }

}

#endif
