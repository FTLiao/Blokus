//
//Blokus OOP�M�D  �q���t:���p�� �³Ӷ�
//
//
//4-26�s�W
//
initial.h ��l��						~/fred/Apr26.cpp  �ϥνd��		

	void GenerateBLOCK(struct BLOCK)	�ͦ����
	void InitialBOARD(struct BOARD)		��l�ƴѽL


//
//4-30�s�W
//		

menu.h �e�ѽL�A�w�]�榡(m,n) = (640,480)			~/fred/Apr30.cpp  �ϥνd��

	void MENU_START(int x,int y)		�ǤJ�ƹ���m�A�P�_�O�_�b�ҿ�d��
						�}�l�C���ﶵ

	void MENU_HISTORY(int x,int y)		���a�O���ﶵ
						
	void MENU_TUTOR(int x,int y)		�C���о�

	void MENU_QUIT(int x,int y)		�C������
	
	void MENU()				draw��l�C���ϥΪ̤���

	int *mouseclick()			�^�Ƿƹ��ʧ@
						x[0] = -1 Default
						       0  Left Click;
						       1  Right Click;
						x[1] = mousex();      x[2] = mousey();
//
//5-02�ק�
//

Boar.h �ѽL���e���T��
	struct PLAYER
	{
  		char         PlayerColor;
		struct BLOCK PlayerPiece[21];
	};
//
//5-05 �W�[����Ӥ��s�W
//

//
//5-06
//
playgraph.h

	void PlayBackground()				ø�e�C���I��
	void PrintPlayerBox(struct PLAYER A[])		�C�����a�����ܳB
menu.h
	bool PLAY_TERMINATE(int,int,int)		�C���O�_�פ�
�ק�	int MENU_START()				�]���C���}�l�᪺main fun
�ק�	void MENU()					�W�߿ﶵhighlight
�s�W	void MenuHighlight(int,int)			Highlight Menu��������

main.cpp	���pø�Ϥ�{��

//
//5-07
//

BoardGraph.h			�D�n��ø�e�ѽL�W���ƥ�

	void PlotBoard(struct BOARD)				�P�_�ѽL�W���C��A�ھڨ�ø�W�Ϯ�
						�D�n�O��readimagefile�ӧ���
	void InitialPlotBoard()			�����մѽL�Ҽ˪�

��ĳ�i�H�լݬ����{��������ѽL�C��e���@�i�ϡA�M���X���@���ɮסA�bŪ�^�ӵ{���̭��A
�Ӥ��O�@�Ӥ@���I�}��Ū�A�o�˩γ\�i�H���Ū���ɶ���runtime

//
//5-08
//
�ק�Ҧ���block


�s�W
GraphData.h	void PlayerBoxData(char x[84][100])		�D�n�x�s�ϧθ��
PlayGraph.h	
	�s�Wvoid PrintMoveImage(char *name,int PieceIndex,int x,int y) �e�X���ʮɪ�����ϧ�

ø��fun �s�W refresh �\��
	�M�w�O�_�n�A�P�O�@����ӵe���A�٬O����Ū���ª��ɮקY�i�C

//
//5-09
//
�s�W
@PlayGame.h
RePrint(struct PLAYER[] , struct BOARD[21][21] , int RefreshOfBox , int RefreshOfBoard)
�Ҷq��PrintMoveImage�A�ݦۦ�[�Wsetvisualpage(page)�A�_�h�[�b�Ƶ{���̪��ܡA�bmove�ɹϷ|�{
@PlayGame.h (124~159) 
put �{���X
�e�ϫܺC�A�|�n�A��ĳ�NBlock.txt���g���m���˦�



�ץ�	�����ɡA�Y���O�bboard�W���A���¥i�H��U�h�����D�C		
	�Q�ΧP�_board��ɥ~������C��ηƹ���m�A�P�w����O�_�bboard��
	�u�I:�]�p�B�P�_²��
	���I:�Y�]�pAI�ɡA���ؤ�k���Ӥ��A��

//
//5-12
//
�s�W	MoveBoundaryCheck	�P�_��U�ɤ���O�_���b�ѽL�~
flip_turn.h
�s�W	void FlipTurnImage(struct BLOCK &b,int playerindex,int pieceindex,char path[50])
	�ǻ�Flipturn �᪺�ϧΦ�m
�ק�	Move Put while�j��
�ק�	PrintMoveImage�����P�O���AŪ���i����

����	����ζ��⪺�ϧ�

//
//5-13 14
//

�s�W
Global.h	�Ҧ��|�ϥΨ쪺�ܼƤΤ�{��

�ק�	
PlayGraph.h	void MoveCo(struct BLOCK &b) �P�_�������
BoardGraph.h	int PutImageCheck()		�P�_�O�_�i�H��m���


================================�|�H�C���Ҧ�����!!!!!!done!!=========================

//
//5-16
//

�ק� playgame �P�_��U������P�_���A�ѩ�if else if �|��̬۪񪺡A�ҥH���M�O�W�@�h��else if�Y�S���[�W�j�A���A�i���ܦ��U�@�hif �������C

//
//5-19
//
�ץ�	������function ���ܱ����V�A�Y�i�ѨM�����m���~�����D

�s�W AI.h
�i�J���j����D AI.idea �Ԩ�

//
//5-23
//
 -lwinmm     for sound 

//
//5-31
//
Board update���D

//
//6-17
//

Debug 
1. setindex��C�|���
2. ���ɭ�AI���|�L�X��
3. AI�|replace (0,0)
4. ESD�S�@��? 0��
5. Fillscore�|��
6. block �|�Qreplace
7. playerbox�ܺC! book b[4][21]


�ѨM�ϧΰ��D!!