#ifndef flip_turn_h
#define flip_turn_h 1

void Flip(struct BLOCK &b)
{
	char t;
	for(int i=0;i<7;i++)
		for(int j=0;j<4;j++)
		{
			t=b.X[i][j];
			b.X[i][j]=b.X[i][6-j];
			b.X[i][6-j]=t;
		}
	list *p,*p2;
	p = b.Dlistp;
	p2 = b.Elistp;
	while(p)
	{
		p->Y = -(p->Y)+6;
		p = p->next;
	}
	while(p2)
	{
		p2->Y = -(p2->Y)+6;
		p2 = p2->next;
	}
	MoveCo(b);
	b.FLIP = (++b.FLIP) %2;
}

void Turn(struct BLOCK &b)
{
	char t;
	int i,j;
	list *p = b.Dlistp,*p2 = b.Elistp;
	if(b.FLIP==0)
	{
		for(i=0;i<4;i++)
			for(j=i;j<6-i;j++)
			{
				t=b.X[i][j];
				b.X[i][j]=b.X[6-j][i];
				b.X[6-j][i]=b.X[6-i][6-j];
				b.X[6-i][6-j]=b.X[j][6-i];
				b.X[j][6-i]=t;
			}		
		while(p)
		{
			i = p->X;
			p->X = p->Y;
			p->Y = -i+6;
			p = p->next;
		}
		while(p2)
		{
			i = p2->X;
			p2->X = p2->Y;
			p2->Y = -i+6;
			p2 = p2->next;
		}
		
	}
	else if(b.FLIP==1)
	{
		for(i=0;i<4;i++)
			for(j=i;j<6-i;j++)
			{
				t=b.X[i][j];
				b.X[i][j]=b.X[j][6-i];
				b.X[j][6-i]=b.X[6-i][6-j];
				b.X[6-i][6-j]=b.X[6-j][i];
				b.X[6-j][i]=t;
			}
		while(p)
		{
			j = p->Y;
			p->Y = p->X;
			p->X = -j+6;
			p = p->next;
		}
		while(p2)
		{
			j = p2->Y;
			p2->Y = p2->X;
			p2->X = -j+6;
			p2 = p2->next;
		}
	}
	MoveCo(b);
	b.TURN = (++b.TURN) %4;
}


void FlipTurnImage(struct BLOCK &b,int playerindex,int pieceindex,char path[50])
{
    void CallString(int,char [],char []);
    char pathtoB[] = "./BlocksPic/Blue/";
    char pathtoY[] = "./BlocksPic/Yellow/";
    char pathtoR[] = "./BlocksPic/Red/";
    char pathtoG[] = "./BlocksPic/Green/";
    pieceindex ++;
    if(playerindex==0)
    {
        strcpy(path,pathtoB);
        CallString(pieceindex,"B-",path);
        CallString(b.TURN,"-",path);
        CallString(b.FLIP,"-",path);
        strcat(path,".gif");
    }
    else if(playerindex==1)
    {
        strcpy(path,pathtoY);
        CallString(pieceindex,"Y-",path);
        CallString(b.TURN,"-",path);
        CallString(b.FLIP,"-",path);
        strcat(path,".gif");
    }
    else if(playerindex==2)
    {
        strcpy(path,pathtoR);
        CallString(pieceindex,"R-",path);
        CallString(b.TURN,"-",path);
        CallString(b.FLIP,"-",path);
        strcat(path,".gif");
    }
    else if(playerindex==3)
    {
        strcpy(path,pathtoG);
        CallString(pieceindex,"G-",path);
        CallString(b.TURN,"-",path);
        CallString(b.FLIP,"-",path);
        strcat(path,".gif");
    }
}

void CallString(int p,char x[],char back[])
{
    strcat(back,x);
    switch(p)
    {
        case 0:
            strcat(back,"0");
            break;
        case 1:
            strcat(back,"1");
            break;
        case 2:
            strcat(back,"2");
            break;
        case 3:
            strcat(back,"3");
            break;
        case 4:
            strcat(back,"4");
            break;
        case 5:
            strcat(back,"5");
            break;
        case 6:
            strcat(back,"6");
            break;
        case 7:
            strcat(back,"7");
            break;
        case 8:
            strcat(back,"8");
            break;
        case 9:
            strcat(back,"9");
            break;
        case 10:
            strcat(back,"10");
            break;
        case 11:
            strcat(back,"11");
            break;
        case 12:
            strcat(back,"12");
            break;
        case 13:
            strcat(back,"13");
            break;
        case 14:
            strcat(back,"14");
            break;
        case 15:
            strcat(back,"15");
            break;
        case 16:
            strcat(back,"16");
            break;
        case 17:
            strcat(back,"17");
            break;
        case 18:
            strcat(back,"18");
            break;
        case 19:
            strcat(back,"19");
            break;
        case 20:
            strcat(back,"20");
            break;
        case 21:
            strcat(back,"21");
            break;
        default:
            break;
    }
}


#endif
