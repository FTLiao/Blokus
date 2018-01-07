#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <graphics.h>
#include <windows.h>
#include <time.h>
#include <math.h>
#include "Global.h"
#include "Initial.h"
#include "menu.h"
#include "PlayGame.h"
#include "PlayGraph.h" 
#include "BoardGraph.h"
#include "flip_turn.h"
#include "AI.h"
#include "Score.h"

int main()
{  
  initwindow(m,n);  
  menupage();
  closegraph();
  return 0;
}
