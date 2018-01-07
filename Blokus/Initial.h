#ifndef Initial_h
#define Initial_h 1

void GenerateBLOCK(struct BLOCK A[])
{
	int i=0,j=0,k=0,count = 0;
	char input;
	struct list *p;
	FILE *fp;
	fp = fopen("Blocks.txt","r");

	if(!fp){
    	printf("NOT open\n");
    	system("pause");
    	exit(1);
  	}

	while(!feof(fp) && (fscanf(fp, " %c" , &input)!=EOF))
	{
		  			
		if(count == 0){
            free(A[k].Dlistp);
            free(A[k].Elistp);            
			A[k].Dlistp = NULL;
			A[k].Elistp = NULL;
		}			

    	A[k].X[i][j] = input;
    	A[k].USE = true;
    	A[k].TURN = 0;
    	A[k].FLIP = 0;
    	if(input=='D')
    	{
			p = (list *)malloc(sizeof(list));
			p->X = i;
			p->Y = j;
			p->next = A[k].Dlistp;
			A[k].Dlistp = p;
		}
    	if(input=='e')
    	{
			p = (list *)malloc(sizeof(list));
			p->X = i;
			p->Y = j;
			p->next = A[k].Elistp;
			A[k].Elistp = p;
		}		 
    	if(j==6)
			i = (++i)%7;
		j = (++j)%7;
		if(k < 2)
			A[k].NO = k + 1;
		else if(k>=2 && k<4)
			A[k].NO = 3;
		else if(k<=8)
			A[k].NO = 4;
		else
			A[k].NO = 5;
		count = (++count)%49;
		if(count == 0)
    		k = (++k)%21;
	}
	fclose(fp);
	
}

void InitialBOARD()
{
  int i,j,k;
  for(i=0;i<21;i++)
    for(j=0;j<21;j++){
      Board[i][j].WHO = '#';
      for(k=0;k<4;k++)
        Board[i][j].BOARDCOLOR[k] = '#';
      }
    Board[0][0].BOARDCOLOR[0] = 'e';//          B   G
    Board[20][0].BOARDCOLOR[1] = 'e';//
    Board[20][20].BOARDCOLOR[2] = 'e';//        Y   R
    Board[0][20].BOARDCOLOR[3] = 'e';//
    setactivepage(3);
    readimagefile("./BlocksPic/initialboard.bmp",mbselect,0,m,n);    
    writeimagefile("BoardUpdate.bmp",mbselect,0,m,n);
}

void BoardToBuffer()
{
    for(int i=0;i<21;i++)
        for(int j=0;j<21;j++)
        {
            if(BufferBoard[i][j].WHO != Board[i][j].WHO)
                BufferBoard[i][j].WHO = Board[i][j].WHO;
            for(int k=0;k<4;k++)
                if(BufferBoard[i][j].BOARDCOLOR[k] != Board[i][j].BOARDCOLOR[k])
                BufferBoard[i][j].BOARDCOLOR[k] = Board[i][j].BOARDCOLOR[k];                
        }
}
void PieceToBuffer()
{
    for(int i=0;i<4;i++)
        for(int j=0;j<21;j++)
        {
            if(A[i].PlayerPiece[j].USE)
                PieceBuffer[i][j] = true;
            else
                PieceBuffer[i][j] = false;
        }
}
void resetmouse()
{
  mouse[0] = mouse[1] = mouse[2] = -1;
}


#endif
