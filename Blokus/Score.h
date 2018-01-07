#ifndef Score_h
#define Score_h 1


void CalScore()
{

	score[playerindex] += A[playerindex].PlayerPiece[pieceindex].NO;
	itoa(score[playerindex],cscore[playerindex],10);
	int setover=0;
	for(int i=0;i<4;i++)
		setover+=A[i].PC_AI;
	printf("setover %d\n",setover);
	if(setover==-4)
	{		
        cleardevice();
        ShowScore();
		GameReset();
		return ;
	}
	
}

void ShowScore()
{
    int winner,number1=0;
	for(int i=0;i<4;i++)
		if(score[i]>number1)
		{
			number1 = score[i];
			winner = i;
		}
	while(1){
	setactivepage(page=-page+1);
	cleardevice();
	setcolor(WHITE);
	settextstyle(10,0,5);
	outtextxy(290,80,"Game Over");
	readimagefile("./BlocksPic/diamond-b.gif",m/2-150,n/2-150,m/2-100,n/2-100);
	readimagefile("./BlocksPic/diamond-y.gif",m/2-150,n/2-50,m/2-100,n/2);
	readimagefile("./BlocksPic/diamond-r.gif",m/2-150,n/2+50,m/2-100,n/2+100);
	readimagefile("./BlocksPic/diamond-g.gif",m/2-150,n/2+150,m/2-100,n/2+200);
	settextstyle(4,0,3);
	setcolor(LIGHTBLUE);
	outtextxy(m/2-80,n/2-135,cscore[0]);
	setcolor(YELLOW);
	outtextxy(m/2-80,n/2-35,cscore[1]);
	setcolor(LIGHTRED);
	outtextxy(m/2-80,n/2+65,cscore[2]);
	setcolor(LIGHTGREEN);
	outtextxy(m/2-80,n/2+165,cscore[3]);
	settextstyle(5,0,4);
	outtextxy(m/2,n/2-135+winner*100,"Winner");
	mouseclick();
	if(mousex()>m-250&&mousey()>n-65&&mousex()<m-125&&mousey()<n-30)
	{
		readimagefile("./BlocksPic/again-on.gif",m-250,n-65,m-150,n-30);
		readimagefile("./BlocksPic/menu-off.gif",m-125,n-65,m-25,n-30);
		if(mouse[0]==0)
		{
			puts("ok");
			GameReset();
			MENU_START();
		}
			
	}	
	else if(mousex()>m-125&&mousey()>n-65&&mousex()<m-25&&mousey()<n-30)
	{
		readimagefile("./BlocksPic/again-off.gif",m-250,n-65,m-150,n-30);		
		readimagefile("./BlocksPic/menu-on.gif",m-125,n-65,m-25,n-30);
		if(mouse[0]==0)
		{
            puts("go back to menu!");
            GameReset();
            menupage();            
        }			
	}	
	else
	{
		readimagefile("./BlocksPic/again-off.gif",m-250,n-65,m-150,n-30);
		readimagefile("./BlocksPic/menu-off.gif",m-125,n-65,m-25,n-30);
	}
			
	setvisualpage(page);
	}
}

void GameReset()
{
	for(int i=0;i<4;i++){
		score[i] = 0;
        itoa(score[i],cscore[i],10);}
}
	
	
	
	
#endif
