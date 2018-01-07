#ifndef menu_h
#define menu_h 1
void menupage()
{
    while(1){
        if(musiccount)
            PlaySound(NULL,NULL,SND_FILENAME);
        else
            PlaySound(musicpath,NULL,SND_ASYNC|SND_FILENAME|SND_LOOP|SND_NOSTOP);
        do{
            mouseclick();
            if(mousebuffer[1]!=mouse[1] || mousebuffer[2]!=mouse[2])
            {            
                setactivepage(page=-page+1);
                MENU();
                MenuHighlight(mousex(),mousey());
                setvisualpage(page);
                mousebuffer[1]=mouse[1];mousebuffer[2]=mouse[2];
            }
        }while(mouse[0]!=0);
        if((mouse[1] < (int)(m/1.5)+150 && mouse[1] > (int)(m/1.5)) && (mouse[2] < n/2 +25 && mouse[2]> n/2))
    	{
        	MENU_START();
    	}
        MENU_SETTING(mouse[1],mouse[2]);
        MENU_TUTOR(mouse[1],mouse[2]);
        MENU_QUIT(mouse[1],mouse[2]);
        _sleep(100);
      }  
}
bool PLAY_TERMINATE()
{
  if(mouse[0] == 0)
    if((mouse[1]<m && mouse[1]>m-55)&&(mouse[2]<n && mouse[2]>n-20))
    {
        cleardevice();
      return true;
    }
    else
      return false;

  return false;
}

void MENU_SETTING(int x,int y)
{
  if((x < (int)(m/1.5)+150 && x > (int)(m/1.5)) && (y < n/2 +30 +25 && y> n/2 + 30))
  {
    cleardevice();
    settextstyle(10,0,4);
    setcolor(LIGHTBLUE);
    outtextxy(m/10,n/2,"Music");
    if(musiccount)
        outtextxy(m/10+150,n/2,"OFF");        
    else
        outtextxy(m/10+150,n/2,"ON!");
    setcolor(WHITE);
    outtextxy(m/10,n/2+200,"Back to menu!");
    musiccount = 0;
    while(1)
    {
        mouseclick();
        SettingHighlight();
        if(mouse[0]==0)
        {
            if(mousex() > m/10+150 && mousex() < m/10+150+60 && mousey() > n/2 && mousey()<n/2+30)
            {
                musiccount = (++musiccount)%2;
                if(musiccount)
                {
                    setcolor(LIGHTBLUE);
                    outtextxy(m/10+150,n/2,"OFF");
                    PlaySound(NULL,NULL,SND_FILENAME);                
                }
                else
                {
                    strcpy(musicpath,"music-1.wav");           
                    setcolor(LIGHTBLUE);
                    outtextxy(m/10+150,n/2,"ON!");
                    PlaySound(musicpath,NULL,SND_ASYNC|SND_FILENAME|SND_LOOP|SND_NOSTOP);                
                }
            }
            if(mousex()>m/10 && mousex()< m/10+250 && mousey()>n/2+200 && mousey() < n/2+200+30)
                break;
        }

    }
  }
}
void MENU_TUTOR(int x,int y)
{
  if((x < (int)(m/1.5)+150 && x > (int)(m/1.5)) && (y < n/2 +60 +25 && y> n/2 +60))
  {
    cleardevice();
    settextstyle(10,0,5);
    setcolor(GREEN);
    outtextxy(m/10,n/2,"NOW WORKING ON!!");
    _sleep(1000);
  }
}
void MENU_QUIT(int x,int y)
{
  if((x < (int)(m/1.5)+150 && x > (int)(m/1.5)) && (y < n/2 +90 +25 && y> n/2 +90))
  {
    cleardevice();
    settextstyle(10,0,5);
    outtextxy(m/10,n/2,"Hope to see U soon!!");
    _sleep(100);
    exit(1);
  }
}

void MENU()
{
  cleardevice();
  setcolor(WHITE);
  settextstyle(10,0,4);
  outtextxy((int)(m/1.5),n/2,"*START");
  outtextxy((int)(m/1.5),n/2+30,"*Setting");
  outtextxy((int)(m/1.5),n/2+60,"*Tutor");
  outtextxy((int)(m/1.5),n/2+90,"*QUIT");
  setcolor(YELLOW);
  settextstyle(10,0,8);
  outtextxy((int)(m/10),n/10,"BLOKUS!!!");
  settextstyle(10,0,3);
  outtextxy((int)(m/10),n/10+60,"C++ Board Game V.0");
  setcolor(WHITE);
  settextstyle(5,0,1);
  outtextxy((int)(m/10),n-40,"Author : Edison & Fred  Copyright Reserved 2012");
}

void mouseclick()
{
  if(ismouseclick(WM_RBUTTONDOWN))  //WM_LBUTTONDOWN ¥kÁä«ö¤U
  {
    clearmouseclick(WM_RBUTTONDOWN);
    mouse[1] = mousex();mouse[2] = mousey();
    mouse[0] = 1;                       //¥kÁä«ö¤U¬°1
  }
  else if(ismouseclick(WM_LBUTTONDOWN))//¥ªÁä«ö¤U
  {
    clearmouseclick(WM_LBUTTONDOWN);
    mouse[1] = mousex();mouse[2] = mousey();
    mouse[0] = 0;                       //¥ªÁä«ö¤U¬°0
  }
  else if(ismouseclick(WM_MBUTTONDOWN))
  {
        clearmouseclick(WM_MBUTTONDOWN);
        mouse[1] = mousex();mouse[2] = mousey();
        mouse[0] = 2;                       //ºu½üÁä«ö¤U¬°2
  }
  else
    {mouse[0] = -1;mouse[1] = mousex();mouse[2] = mousey();}//Default value if mouse buttoms were not clicked
}

void MenuHighlight(int x,int y)
{
  if((x < (int)(m/1.5)+150 && x > (int)(m/1.5)) && (y < n/2 +25 && y> n/2)){
     setcolor(YELLOW);
     settextstyle(10,0,4);
     outtextxy((int)(m/1.5),n/2,"*START");
  }
  else{
     setcolor(WHITE);
     settextstyle(10,0,4);
     outtextxy((int)(m/1.5),n/2,"*START");
  }


  if((x < (int)(m/1.5)+150 && x > (int)(m/1.5)) && (y < n/2 +30 +25 && y> n/2 +30)){
     setcolor(LIGHTBLUE);
     settextstyle(10,0,4);
    outtextxy((int)(m/1.5),n/2+30,"*Setting");
  }
  else{
     setcolor(WHITE);
     settextstyle(10,0,4);
    outtextxy((int)(m/1.5),n/2+30,"*Setting");
  }

  if((x < (int)(m/1.5)+150 && x > (int)(m/1.5)) && (y < n/2 +60 +25 && y> n/2 +60)){
     setcolor(LIGHTRED);
     settextstyle(10,0,4);
    outtextxy((int)(m/1.5),n/2+60,"*Tutor");
  }
  else{
     setcolor(WHITE);
     settextstyle(10,0,4);
    outtextxy((int)(m/1.5),n/2+60,"*Tutor");
  }
  if((x < (int)(m/1.5)+150 && x > (int)(m/1.5)) && (y < n/2 +90 +25 && y> n/2 +90)){
     setcolor(LIGHTGREEN);
     settextstyle(10,0,4);
     outtextxy((int)(m/1.5),n/2+90,"*QUIT");
  }
  else{
     setcolor(WHITE);
     settextstyle(10,0,4);
    outtextxy((int)(m/1.5),n/2+90,"*QUIT");
  }
}
void SettingHighlight()
{
    //music setting
    if(mousex() > m/10+150 && mousex() < m/10+150+60 && mousey() > n/2 && mousey() <n/2+30)
    {
        setcolor(WHITE);
        rectangle(m/10+150,n/2,m/10+150+60,n/2+30);
    }
    else
    {
        setcolor(BLACK);
        rectangle(m/10+150,n/2,m/10+150+60,n/2+30);
    }
    
    //back to menu
    if(mousex()>m/10 && mousex()< m/10+250 && mousey()>n/2+200 && mousey() < n/2+200+30)
    {
        setcolor(WHITE);
        rectangle(m/10,n/2+200,m/10+250,n/2+200+30);
        //readimagefile("./BlocksPic/Command/menu-1.gif",m/10,n/2+200,m/10+138/2,n/2+200+59/2);
    }
    else
    {
        setcolor(BLACK);
        rectangle(m/10,n/2+200,m/10+250,n/2+200+30);
        //readimagefile("./BlocksPic/Command/menu.gif",m/10,n/2+200,m/10+138/2,n/2+200+59/2);
    }
}
#endif
