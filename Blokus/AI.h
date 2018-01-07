#ifndef AI_h
#define AI_h 1

class CALNEXT
{
		
	int nowblank;
	BOARD futureb[21][21];
public:
	int ESDscore;
	int Edgescore;
	int Fillscore;
	int TotalScore;
	CALNEXT(){
			
		for(int i=0;i<21;i++)
        	for(int j=0;j<21;j++){
            	futureb[i][j].WHO= Board[i][j].WHO;
            	for(int k=0;k<4;k++)
               		futureb[i][j].BOARDCOLOR[k] = Board[i][j].BOARDCOLOR[k];                
        	}        	
	}
	void reset()
	{
		for(int i=0;i<21;i++)
        	for(int j=0;j<21;j++){
				if(futureb[i][j].WHO != Board[i][j].WHO)
            		futureb[i][j].WHO= Board[i][j].WHO;
            	for(int k=0;k<4;k++)
            		if(futureb[i][j].BOARDCOLOR[k] != Board[i][j].BOARDCOLOR[k])
               			futureb[i][j].BOARDCOLOR[k] = Board[i][j].BOARDCOLOR[k];                
        	}
	}
	void ESD(const int &,const int &,const int &);
	void Edge(const int &,const int &,const int &);
	void CalBlank(const int ,const int );
	void Fill(const int &,const int &,const int &);
	void PutOn(const int &,const int &,const int &);		
};

void CALNEXT::PutOn(const int &piece,const int &iput, const int &jput)
{
	for(int i=0;i<7;i++)
		for(int j=0;j<7;j++)
        {
			if((iput+i-3 >=0) && (jput+j-3 >=0)&&(iput+i-3 <=20) && (jput+j-3 <=20))
			{
				if(A[playerindex].PlayerPiece[piece].X[i][j] == 'D'){
					futureb[iput+i-3][jput+j-3].WHO = CharOnBoard[playerindex];
					futureb[iput+i-3][jput+j-3].BOARDCOLOR[playerindex] = 'D';
    	        }
				else if(A[playerindex].PlayerPiece[piece].X[i][j] == 'd'){
					futureb[iput+i-3][jput+j-3].WHO = CharOnBoard[playerindex];
					futureb[iput+i-3][jput+j-3].BOARDCOLOR[playerindex] = 'd';
    	        }    	        
				else if(A[playerindex].PlayerPiece[piece].X[i][j] == 's'){
					if(toupper(futureb[iput+i-3][jput+j-3].BOARDCOLOR[playerindex]) != 'D')
						futureb[iput+i-3][jput+j-3].BOARDCOLOR[playerindex] = 's';
    	           }
				else if(A[playerindex].PlayerPiece[piece].X[i][j] == 'e'){
					if(futureb[iput+i-3][jput+j-3].BOARDCOLOR[playerindex] == '#')
						futureb[iput+i-3][jput+j-3].BOARDCOLOR[playerindex] = 'e';
				}
			}
		
		}		
	//�p��futureboard ESD,Eage,Fill�����ƫ�s��Totalscore
	ESD(piece,iput,jput);
	Edge(piece,iput,jput);
	Fill(piece,iput,jput);
	if(setindex >=0)
	    TotalScore = ESDscore*10 + Edgescore*4 + Fillscore*0 + (int)pow(A[playerindex].PlayerPiece[piece].NO,3);
	else if(setindex>=7)
	    TotalScore = ESDscore*15 + Edgescore*4 + Fillscore/5 + (int)pow(A[playerindex].PlayerPiece[piece].NO,3);
	else if(setindex>=10)
        TotalScore = ESDscore*10 + Edgescore*4 + Fillscore/3 + (int)pow(A[playerindex].PlayerPiece[piece].NO,3);
	else if(setindex >=14)
		TotalScore = ESDscore*5 + Edgescore*4 + Fillscore + A[playerindex].PlayerPiece[piece].NO*10;
}


void CALNEXT::ESD(const int &piece,const int &iput,const int &jput)
{
	list *p = A[playerindex].PlayerPiece[piece].Dlistp;	
	ESDscore = 0;
	while(p){
		for(int player = playerindex+1; player!=playerindex ; player = (++player)%4)
        {
			if(futureb[iput-3+p->X][jput-3+p->Y].BOARDCOLOR[player] == 's')
            {
				for(int j=0;j<4;j++)
                {
					if(futureb[iput-3+p->X+pos[j][0]][jput-3+p->Y+pos[j][1]].BOARDCOLOR[player] == 'D')
                    {
						if(futureb[iput-3+p->X+pos[(j+1)%4][0]][jput-3+p->Y+pos[(j+1)%4][1]].BOARDCOLOR[player] == 'e')
							ESDscore++;
						if(futureb[iput-3+p->X+pos[(j+3)%4][0]][jput-3+p->Y+pos[(j+3)%4][1]].BOARDCOLOR[player] == 'e')
							ESDscore++;
					}
				}
			}
		}
		p = p->next;
	}
}

void CALNEXT::Edge(const int &piece,const int &iput,const int &jput)
{
	list *p = A[playerindex].PlayerPiece[piece].Elistp;
	Edgescore = 0;
	while(p)
	{
		if(futureb[iput-3+p->X][jput-3+p->Y].WHO == '#')
			Edgescore++;
		p = p->next;
	}
}

void CALNEXT::CalBlank(const int mx,const int my)
{
	int nearx,neary;
	if(nowblank>99)
		return ;
	for(int j=0;j<4;j++)
	{
		nearx = mx + pos[j][0];
		neary = my + pos[j][1];
		if(nearx>=0 && nearx<21 && neary>=0 && neary<21)
		{
			if(futureb[nearx][neary].WHO == '#')
			{
				nowblank++;
				CalBlank(nearx,neary);
			}
		}
	}
	
}
		

void CALNEXT::Fill(const int &piece,const int &iput,const int &jput)
{	
	Fillscore = 0;
	if(setindex>=7)
	{
		list *p = A[playerindex].PlayerPiece[piece].Elistp;
		while(p)
		{
            nowblank = 0;
			if(futureb[iput-3+p->X][jput-3+p->Y].WHO == '#')
			{
				
				CalBlank(iput-3+p->X,jput-3+p->Y);
			}
			if(nowblank > Fillscore)
				Fillscore = nowblank;
			if(Fillscore > 100){
				break;}
			p = p->next;
		}
	}

}	


void On_End(int iA,int jA)
{
	list *p;
    for(int i=0;i<7;i++)
		for(int j=0;j<7;j++)
        {							                           
            if((iA+i-3 >=0) && (jA+j-3 >=0)&&(iA+i-3 <=20) && (jA+j-3 <=20))
            {
			    if(A[playerindex].PlayerPiece[pieceindex].X[i][j] == 'D'){
        			Board[iA+i-3][jA+j-3].WHO = CharOnBoard[playerindex];
		          	Board[iA+i-3][jA+j-3].BOARDCOLOR[playerindex] = 'D';
                }
                else if(A[playerindex].PlayerPiece[pieceindex].X[i][j] == 'd'){
        			Board[iA+i-3][jA+j-3].WHO = CharOnBoard[playerindex];
		          	Board[iA+i-3][jA+j-3].BOARDCOLOR[playerindex] = 'd';
                }
        		else if(A[playerindex].PlayerPiece[pieceindex].X[i][j] == 's'){
				    if(toupper(Board[iA+i-3][jA+j-3].BOARDCOLOR[playerindex]) != 'D')
						Board[iA+i-3][jA+j-3].BOARDCOLOR[playerindex] = 's';
                }
		        else if(A[playerindex].PlayerPiece[pieceindex].X[i][j] == 'e'){
					if(Board[iA+i-3][jA+j-3].BOARDCOLOR[playerindex] == '#')
					{
									p = (list*)malloc(sizeof(list));
									p->X = iA+i-3;
									p->Y = jA+j-3;
									p->next = Eonboard[playerindex];
									Eonboard[playerindex] = p;									
									Board[iA+i-3][jA+j-3].BOARDCOLOR[playerindex] = 'e';
					}
                }
			}
        }
    A[playerindex].PlayerPiece[pieceindex].USE = false;    
    lastpiece[playerindex] = pieceindex;
    GameOver();
    CalScore();
}
void FindEage()
{
	int cA=10,cB=10;
	do{		
		for(int i=cA ; i<=cB ;i++)
		{
			if(Board[i][cA].BOARDCOLOR[playerindex] == 'e')      //���ƦV�U 
			{	edgex = i;edgey = cA; return ;}
			else if(Board[cA][i].BOARDCOLOR[playerindex] == 'e')//�W�ƦV�k 
			{	edgex = cA;edgey = i; return ;}
			else if(Board[i][ cB ].BOARDCOLOR[playerindex] == 'e')//�k�ƦV�U 
			{	edgex = i;edgey = cB; return ;}
			else if(Board[cB][i].BOARDCOLOR[playerindex] == 'e')//�U�ƦV�k 
			{	edgex = cB;edgey = i; return ;}
		}                                                         //edgex����row edgey����column 
		cA--;//�����W�� 
		cB++;//���k�U 
	}while(cA>=0);	
}
int RandPut()  //�}�����H����m 
{
    if(Board[10][10].WHO != '#' && setindex == 3)
    {
		puts("middle has b");
        Period_B();
	}
	
    else{
	int w=0;       
	FindEage();
    while(setindex<6){
    	pieceindex = rand()%6+15;
       	if(pieceindex==18)
       		continue;
		else if(A[playerindex].PlayerPiece[pieceindex].USE == true)
       	    break;
    }
    MoveCo(A[playerindex].PlayerPiece[pieceindex]);   
    int iAput,jAput;
    int fl,tu,pc,get = 0;
    struct list *dis;
    for(int iA=edgex-7;iA<edgex+7;iA++)            //iA,jA���Piput jput���q���ƹ�
        for(int jA=edgey-7;jA<edgey+7;jA++)
        {
			if(!MoveBoundaryCheck(iA,jA))
			     continue;
            for(int f=0;f<2;f++)
            {
                for(int t=0;t<4;t++)
                {
					if( !MoveBoundaryCheck(iA,jA)){
                        Turn(A[playerindex].PlayerPiece[pieceindex]);
						continue;
                    }
					if( PutImageCheck(iA,jA)!=2  ){//�}�������������P�_ 
						Turn(A[playerindex].PlayerPiece[pieceindex]);
						continue;
					}
					else if( !PutImageCheck(iA,jA)){
                        Turn(A[playerindex].PlayerPiece[pieceindex]);
                        continue;
                    }
                    get = 0;
					if((edgex-10 < 0 && edgey-10 >0) && (iA+movexy[1]==edgex && jA+movexy[2] == edgey)&&A[playerindex].PlayerPiece[pieceindex].X[movexy[1]+3][movexy[2]+3]=='D')//�ѤW���Ĥ@�H�� 
					{
					   get = 1;					                     
                    }
					else if((edgex-10 < 0 && edgey-10 <0)&& (iA+movexy[1]==edgex && jA+movexy[0] == edgey)&&A[playerindex].PlayerPiece[pieceindex].X[movexy[1]+3][movexy[0]+3]=='D')//���W���ĤG�H�� 
					{
					   get = 1;
                    }
					else if((edgex-10 > 0 && edgey-10 <0)&& (iA+movexy[3]==edgex && jA+movexy[0] == edgey)&&A[playerindex].PlayerPiece[pieceindex].X[movexy[3]+3][movexy[0]+3]=='D')//���U���ĤT�H�� 
					{
					   get = 1;
                    }
					else if((edgex-10 > 0 && edgey-10 >0)&& (iA+movexy[3]==edgex && jA+movexy[2] == edgey)&&A[playerindex].PlayerPiece[pieceindex].X[movexy[3]+3][movexy[2]+3]=='D')//�k�U���ĥ|�H�� 
					{
					   get = 1;
                    }
					else
					{
                        Turn(A[playerindex].PlayerPiece[pieceindex]);
                        continue;
                    }
                    if(get)
                    {
                        fl = A[playerindex].PlayerPiece[pieceindex].FLIP;
                        tu = A[playerindex].PlayerPiece[pieceindex].TURN;
                        iAput = iA;jAput = jA;                                                
                    }
                    Turn(A[playerindex].PlayerPiece[pieceindex]);
                                                                               
                }
                Flip(A[playerindex].PlayerPiece[pieceindex]);
            }
        }
        
    while(A[playerindex].PlayerPiece[pieceindex].TURN != tu)
		Turn(A[playerindex].PlayerPiece[pieceindex]);
	if(A[playerindex].PlayerPiece[pieceindex].FLIP != fl)
		Flip(A[playerindex].PlayerPiece[pieceindex]); 
    On_End(iAput,jAput);
		
	RePrint(A,1,1);
	blockhighlight(iAput,jAput);	
	setvisualpage(page);

	BoardToBuffer();
	PieceToBuffer();
	
    playerindex = (++playerindex) %4;
    if(playerindex == 0)
		setindex++;
    }
    return 0;//�񧹴N�����F!            
}
void Period_B()
{
	
	list *Ep = Eonboard[playerindex];
	list *Dp,*testDp;
	int piece,t,f,iput,jput,maxscore[6] = {0};
	
	class CALNEXT nowB;
    while(Ep)//�C��Board�W��Edge 
	{
		if(Ep==NULL) break;
		if(Board[Ep->X][Ep->Y].BOARDCOLOR[playerindex]=='e')
        {
			for(piece=20;piece>-1;piece--)
            {//4��H�W��piece
				if(A[playerindex].PlayerPiece[piece].USE == false)
					continue;				
				for(t=0;t<4;t++)
                {
					for(f=0;f<2;f++)
                    {
                        Flip(A[playerindex].PlayerPiece[piece]);						
						Dp = A[playerindex].PlayerPiece[piece].Dlistp;

						while(Dp)
                        {//�C��D 
							iput = Ep->X - Dp->X + 3;
							jput = Ep->Y - Dp->Y + 3;
							pieceindex = piece;
							if(!MoveBoundaryCheck(iput,jput))
							{
								Dp = Dp->next;
								continue;
							}
							else if(PutImageCheck(iput,jput)!=1)
							{
								Dp = Dp->next;
								continue;
							}
							else
							{
								nowB.reset();
								nowB.PutOn(piece,iput,jput);
								//�p��nowB�����ơA�Y���ư��h�O�Upiece��y�� 	
								if(nowB.TotalScore > maxscore[0])
								{
									maxscore[0] = nowB.TotalScore;
									printf(" Total: %d ",maxscore[0]);
                                    printf("ESD Eadge Fill block %d %d %d %d\n",nowB.ESDscore,nowB.Edgescore,nowB.Fillscore,A[playerindex].PlayerPiece[piece].NO); 
									maxscore[1] = piece;
									maxscore[2] = iput;
									maxscore[3] = jput;
									maxscore[4] = A[playerindex].PlayerPiece[piece].TURN;
									maxscore[5] = A[playerindex].PlayerPiece[piece].FLIP;
								}								
							}
                            Dp = Dp->next;	
						}
					}
					Turn(A[playerindex].PlayerPiece[piece]);
				}
				
			}		
		}
		Ep = Ep->next;
	}
	pieceindex = maxscore[1];
	while(A[playerindex].PlayerPiece[pieceindex].TURN != maxscore[4])
		Turn(A[playerindex].PlayerPiece[pieceindex]);
	if(A[playerindex].PlayerPiece[pieceindex].FLIP != maxscore[5])
		Flip(A[playerindex].PlayerPiece[pieceindex]);
		
	On_End(maxscore[2],maxscore[3]);
	    
	RePrint(A,1,1);
	blockhighlight(maxscore[2],maxscore[3]);
	setvisualpage(page);
	
    BoardToBuffer();
	PieceToBuffer();
    	             
    playerindex = (++playerindex) %4;
    if(playerindex == 0)
		setindex++;	
}

void GameOver()
{
    struct list *Dp;
    struct list *Ep;
    int iG,jG;
    int currentpiece = pieceindex;
    if(setindex > 10)
    { //�j��ĤQ���~�}�l�� 
	    for(int piece=0;piece<21;piece++)
	    {
	        if(A[playerindex].PlayerPiece[piece].USE == false)
	            continue;
	        pieceindex = piece;
	        Ep = Eonboard[playerindex];
	        while(Ep)
	        {
	            for(int f=0;f<2;f++)
	            {
	                for(int t=0;t<4;t++)
	                {
                    
	                    Turn(A[playerindex].PlayerPiece[piece]);
    	                Dp = A[playerindex].PlayerPiece[piece].Dlistp;
    	                while(Dp)
    	                {
    	                    iG = Ep->X - Dp->X + 3;
							jG = Ep->Y - Dp->Y + 3;
							if(!MoveBoundaryCheck(iG,jG)){						
									Dp = Dp->next;
									continue;}						
							else if(PutImageCheck(iG,jG)!=1){     //���i�H��N�~�� 
									Dp = Dp->next;
									continue;}
							else
							{
    	                        while(A[playerindex].PlayerPiece[piece].TURN)
    	                            Turn(A[playerindex].PlayerPiece[piece]);
    	                        while(A[playerindex].PlayerPiece[piece].FLIP)
    	                            Flip(A[playerindex].PlayerPiece[piece]);
    	                        pieceindex = currentpiece;
    	                        return ;
	                        }
        	                Dp=Dp->next;		                    
						                                  //���o�X�P�O���N��i�H��                         
            	        }
                	}
                	Flip(A[playerindex].PlayerPiece[piece]);
            	}
            	Ep=Ep->next;
        	}        
    	}      
    	if(lastpiece[playerindex]==0)
    	   score[playerindex]+=5;
    	A[playerindex].PC_AI = -1;  //���񪱤F && ������� 
    }
    else                        //���X�Ҧ��P�O���N��S���i�H�񪺤F !! 
        return ; 
}
#endif
