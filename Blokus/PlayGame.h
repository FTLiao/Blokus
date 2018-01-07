#ifndef PlayGame_h
#define PlayGame_h 1
int MENU_START()
{

		puts("start");
		int mode= SelectMode();
		
		while(mode == 1)
		{
			InitialMode1();
			while(1)
			{
				if(A[playerindex].PC_AI == -1){
					playerindex = (++playerindex) %4;
                    if(playerindex == 0)
			             setindex++;
					continue;
				}
				//玩家	
				while(A[playerindex].PC_AI == 0)
				{					
					if( PlayerSelect() == 0)
						return 0;			//選到play terminate		
					PlayerPut();		
				}
				//AI
				if(A[playerindex].PC_AI == 1)//||A[playerindex].PC_AI == 0
				{
					printf("set %d\n",setindex);
					if(setindex<3)
						RandPut();
					else
						Period_B();	
//					system("pause");
				}
				
			}		
		}
		InitialSetting();
		A[0].PC_AI = A[1].PC_AI = A[2].PC_AI = A[3].PC_AI = 0;
		//
		//Multi-player start 進入遊戲的loop
		//
		list *p;
		while(mode == 2)
        {
            if(A[playerindex].PC_AI == -1){
					outtextxy(6*m/5,50,"Can not put blocks anymore.");
					_sleep(500);
					playerindex = (++playerindex) %4;
                    if(playerindex == 0)
			             setindex++;
					continue;
				}
			else
			{
			     if( PlayerSelect() == 0)
    				return 0;
                 PlayerPut();
            }
		}
		//
		//Multi-player end
		//
  //}
	//
	//if end
	//
	return 0;
}
int SelectMode()
{
	setactivepage(page=-page+1);
	cleardevice();
	puts("select");
	while(1)
	{
		mouseclick();
		
		readimagefile("./BlocksPic/selectmode.gif",m/2-125,125,m/2+125,200);
		if(mousex()>m/2-80 && mousex()<m/2+80 && mousey()>n/2-25-40 && mousey()<n/2+25-40)
		{
			readimagefile("./BlocksPic/Command/mode-single-on.gif",m/2-80,n/2-25-40,m/2+80,n/2+25-40);
			if(mouse[0] == 0)
				return 1;
				
		}
		else if(mousex()>m/2-80 && mousex()<m/2+80 && mousey()>n/2-25+40 && mousey()<n/2+25+40)
		{
			readimagefile("./BlocksPic/Command/mode-multi-on.gif",m/2-80,n/2-25+40,m/2+80,n/2+25+40);
			if(mouse[0] == 0)
				return 2;
		}
		else
		{
			readimagefile("./BlocksPic/Command/mode-single.gif",m/2-80,n/2-25-40,m/2+80,n/2+25-40);
			readimagefile("./BlocksPic/Command/mode-multi.gif",m/2-80,n/2-25+40,m/2+80,n/2+25+40);
		}
		setvisualpage(page);
	}
	
}

void InitialSetting()
{

		//初始化玩家
		A[0].PlayerColor = 'B';
		A[1].PlayerColor = 'Y';
		A[2].PlayerColor = 'R';
		A[3].PlayerColor = 'G';
		for(int i=0;i<4;i++)
		{
			GenerateBLOCK(A[i].PlayerPiece);		
			free(Eonboard[i]);
			Eonboard[i] = NULL;
			score[i] = 0;
        }
        
        
		playerindex=0;
		setindex=0;
		
		resetmouse();		
		for(int i=0;i<4;i++)
		{
            setactivepage(i);
		  readimagefile("BKimage-board.bmp",0,0,m,n);
        }

		InitialBOARD();//初始化棋盤
		InitialPlayerBox();	
		
		BoardToBuffer();
		PieceToBuffer();
        	
		setactivepage(2);
		setcolor(LIGHTGREEN);
		settextstyle(5,0,4);		
        setbkcolor(BLACK);
		outtextxy(m/2,n-100,"Now Processing.........");
		setvisualpage(2);
		
        _sleep(1500);
        GameReset();
		RePrint(A,0,0);
//		readimagefile("BKimage-board.bmp",mbselect+1,n-nbside+2,m,n);
		setvisualpage(page);
		
}
void InitialMode1()
{
			setbkcolor(CYAN);
            cleardevice();
			srand(time(NULL));
			readimagefile("./BlocksPic/Command/cBLUE-1.gif",m/2-80,125,m/2+80,175);
			readimagefile("./BlocksPic/Command/cYELLOW-1.gif",m/2-80,225,m/2+80,275);
			readimagefile("./BlocksPic/Command/cRED-1.gif",m/2-80,325,m/2+80,375);
			readimagefile("./BlocksPic/Command/cGREEN-1.gif",m/2-80,425,m/2+80,475);
			A[0].PC_AI = A[1].PC_AI = A[2].PC_AI = A[3].PC_AI = 1;
			
			//select color
			while(1)
			{				
				mouseclick();				
				if(mousex()>m/2-80 && mousex()<m/2+80 && mousey()>125 && mousey()<175){
					readimagefile("./BlocksPic/Command/cBLUE-2.gif",m/2-80,125,m/2+80,175);
					if(mouse[0] == 0){
//						A[0].PC_AI = 0;
						A[0].PC_AI = 1;
						break;
					}				
				}
				else if(mousex()>m/2-80 && mousex()<m/2+80 && mousey()>225 && mousey()<275){
					readimagefile("./BlocksPic/Command/cYELLOW-2.gif",m/2-80,225,m/2+80,275);
					if(mouse[0] == 0){
						A[1].PC_AI = 0;
						break;
					}
				}
				else if(mousex()>m/2-80 && mousex()<m/2+80 && mousey()>325 && mousey()<375){
					readimagefile("./BlocksPic/Command/cRED-2.gif",m/2-80,325,m/2+80,375);
					if(mouse[0] == 0){
						A[2].PC_AI = 0;
						break;
					}
				}
				else if(mousex()>m/2-80 && mousex()<m/2+80 && mousey()>425 && mousey()<475){
					readimagefile("./BlocksPic/Command/cGREEN-2.gif",m/2-80,425,m/2+80,475);
					if(mouse[0] == 0){
						A[3].PC_AI = 0;
						break;
					}
				}
				else{
					readimagefile("./BlocksPic/Command/cBLUE-1.gif",m/2-80,125,m/2+80,175);
					readimagefile("./BlocksPic/Command/cYELLOW-1.gif",m/2-80,225,m/2+80,275);
					readimagefile("./BlocksPic/Command/cRED-1.gif",m/2-80,325,m/2+80,375);
					readimagefile("./BlocksPic/Command/cGREEN-1.gif",m/2-80,425,m/2+80,475);
				}
			}			
			InitialSetting();	
}



int PlayerSelect()
{
    RePrint(A,3,0);
	setvisualpage(page);
	while(1){
				mouseclick();
				if(mousebuffer[0]!= mouse[0] || mousebuffer[1]!=mouse[1] || mousebuffer[2]!=mouse[2])
				{
				    RePrint(A,0,0);
                    setvisualpage(page);
	                BoxHighlight(mousex(),mousey());
	                mousebuffer[0]=-1;mousebuffer[1]=mouse[1];mousebuffer[2]=mouse[2];
                }
				if(mouse[0]==0)
				{
					if(PLAY_TERMINATE())
					{
		                for(int i=0;i<4;i++)
		              	   GenerateBLOCK(A[i].PlayerPiece);
		              	GameReset();
						return 0;
				    }
				    else if(mouse[1]<m*7/20 && 150*playerindex<mouse[2] && mouse[2]<150*(playerindex+1))
					{
						pieceindex = mouse[2]%150/50*7 + mouse[1]/40;
						if(A[playerindex].PlayerPiece[pieceindex].USE)
						{
                            A[playerindex].PlayerPiece[pieceindex].USE = false;
                            FlipTurnImage(A[playerindex].PlayerPiece[pieceindex],playerindex,pieceindex,path);
                            MoveCo(A[playerindex].PlayerPiece[pieceindex]);
                   			RePrint(A,2,0);
     		                setvisualpage(page);
    		  	       	    return 1;
                        }
					}
				}
            }
}

void PlayerPut()
{
	list *p = NULL;
	int iput,jput;
	//
	//move and put loop Start
	//
	while(1)
    {
		mouseclick();
		//是否按下放棄按鈕
		if(mouse[0]==0&&QuitBlockCheck()){
        	A[playerindex].PlayerPiece[pieceindex].USE = true;			
        	while(A[playerindex].PlayerPiece[pieceindex].TURN)
        		Turn(A[playerindex].PlayerPiece[pieceindex]);
            A[playerindex].PlayerPiece[pieceindex].TURN = 0;
            if(A[playerindex].PlayerPiece[pieceindex].FLIP)
            	Flip(A[playerindex].PlayerPiece[pieceindex]);	
            A[playerindex].PlayerPiece[pieceindex].FLIP = 0;
            
			RePrint(A,3,0);
        	setvisualpage(page);
    		break;
        }
		//右鍵旋轉
		else if(mouse[0] == 1){
			Turn(A[playerindex].PlayerPiece[pieceindex]);
            FlipTurnImage(A[playerindex].PlayerPiece[pieceindex],playerindex,pieceindex,path);
		}
		//中鍵flip
		else if(mouse[0] == 2){
        	Flip(A[playerindex].PlayerPiece[pieceindex]);
            FlipTurnImage(A[playerindex].PlayerPiece[pieceindex],playerindex,pieceindex,path);
        }
		//put
		//棋盤內放置
		else if(mouse[0]==0 && cnr[0][0]<mouse[1] && mouse[1]<cnr[0][1] && cnr[1][0]<mouse[2] && mouse[2]<cnr[1][1])
        {
			iput = (mouse[2]-cnr[1][0])/s;
			jput = (mouse[1]-cnr[0][0])/s;
			//if超出棋盤
			if( !MoveBoundaryCheck(iput,jput) ){
				setcolor(LIGHTRED);
				outtextxy(m/2,24,"Out Of Board!!");
				_sleep(500);
				continue;
			}
			//if放置錯誤
			else if( !PutImageCheck(iput,jput) ){
				setcolor(LIGHTRED);
				outtextxy(m/2,24,"Not Available");
				_sleep(500);
				continue;
			}
			else if(playerindex > -1)
            {
				On_End(iput,jput);
			}
            writeimagefile("PlayGame.bmp",0,0,m,n);
            setactivepage(2);
			readimagefile("PlayGame.bmp",0,0,m,n);
			readimagefile("./BlocksPic/Command/nextplayer.gif",(cnr[0][0]+cnr[0][1])/2-189,n/2,(cnr[0][0]+cnr[0][1])/2+189,n/2+46);
            setvisualpage(2); 
                
			RePrint(A,1,1);
			blockhighlight(iput,jput);
			setvisualpage(page);
			
			BoardToBuffer();
			PieceToBuffer();
			         
			playerindex = (++playerindex)%4;
            if(playerindex == 0)
                setindex++;
			
			break;
		}
		//
		//put end
		//put
		if(mousebuffer[0]!= mouse[0] || mousebuffer[1]!=mouse[1] || mousebuffer[2]!=mouse[2])
		{
    		RePrint(A,0,0);
        	HighLightEdge(playerindex);
        	PrintMoveImage(path,pieceindex,mousex(),mousey(),A[playerindex].PlayerPiece[pieceindex].TURN);
    	    QuitHighlight();
    		setvisualpage(page);
    		mousebuffer[0]=-1;mousebuffer[1]=mouse[1];mousebuffer[2]=mouse[2];
        }
        
	}
	//
	//move and put loop end
	//

}
#endif
