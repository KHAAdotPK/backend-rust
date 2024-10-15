/*
    src/native/main.c
    aEon@khaa.pk
 */

/*
    https://stackoverflow.com/questions/29172591/c-function-fwrite-doesnt-write-in-file
 */

/*
    Being photogenic
    https://www.youtube.com/shorts/05vNHDXx_bo
 */

#include "main.h"

/*
    Scan-Converting a character
 */
void bitmap_font(unsigned char* pixels, unsigned int height, unsigned int width)
{
    
}

void draw_circle(unsigned char* pixels, unsigned int height, unsigned int width, unsigned int a, unsigned int b, unsigned int r, unsigned char color) {
 
    //We'll start at the top of the circle(generating all the pixel coordinates in 90^. to 45^.) 
    //First point is always on the circle so error is zero and we know that x is zero and y is r       	  
    unsigned int x = 0, y = r;
    /*
        - Distance from origin at (xi, yi), this is your last scaned pixel        
        - Distance from origin to pixel T
          D(T)=(xi + 1)^2 + yi^2 - r^2
        - Distance from origin to pixel S
          D(S)=(xi + 1)^2 + (yi - 1)^2 - r^2
        - The funtion D provides a relative mesurement of the distance from the center of a pixel to the true circle.
          D(T) will always be positive(T is outside the true circle) and D(S) will always be negative(S is inside the true circle)
        - Decision has to be made either move in x direction one unit or move in x direction one unit and move in y direction one negative unit 
        - Our decition variable would be
          di = D(T) + D(S) and for starting pixel coordinates (0, r) it would be di = 3 - 2r
          For the next pixel the decision variable would be di+1 = 2(xi+1 + 1)^2 + y^2i+1 + (yi+1 - 1)^2 - 2r^2
          di+1 - di = 2(xi+1 + 1)^2 + y^2i+1 + (yi+1 - 1)^2 - 2r^2 - (D(T) + D(S))
          Since xi+1 = (xi + 1) and yi+1 = yi then di+1 = di + 4xi + 6 and T is chosen (di < 0)
          Since yi+1 = yi - 1 and so di+1 = di + 4(xi - yi) + 10 for S to be chosen (di > 0)
     */
    int d = (3 - 2*r);
 
    // x is initially zero, x will be same as y at 45(degree) angle
    while( x <= y ) {
         
        // Eight way symmetry of circle
	    //setPixelColor(cxt, y + a, x + b, color);
        //pixels[(x + b)*width + (y + a)] = 2; 
        pixels[(y + b)*width + (x + a)] = color;

        //setPixelColor(cxt, x + a, y + b, color);
        //pixels[(y + b)*width + (x + a)] = 2;
        pixels[(x + b)*width + (y + a)] = color;

	    //setPixelColor(cxt, x + a, (-1)*y + b, color);    
        //pixels[((-1)*y + b)*width + (x + a)] = color;
        pixels[(y + b)*width + (-x + a)] = color;

	    //setPixelColor(cxt, y + a, (-1)*x + b, color);
        //pixels[((-1)*x + b) + (y + a)] = color;
        pixels[(x + b)*width + (-y + a)] = color;

	    //setPixelColor(cxt, (-1)*y + a, (-1)*x + b, color);
        //pixels[((-1)*x + b)*width + ((-1)*y + a)] = color;
        pixels[(-y + b)*width + (-x + a)] = color;

	    //setPixelColor(cxt, (-1)*x + a, (-1)*y + b, color);
        //pixels[((-1)*y + b)*width + ((-1)*x + a)] = color;
        pixels[(-x + b)*width + (-y + a)] = color;

	    //setPixelColor(cxt, (-1)*x + a, y + b, color);
        //pixels[(y + b)*width + ((-1)*x + a)] = color;
        pixels[(-y + b)*width + (x + a)] = color;

	    //setPixelColor(cxt, (-1)*y + a, x + b, color);
        //pixels[(x + b)*width + ((-1)*y + a)] = color;
        pixels[(-x + b)*width + (y + a)] = color;
	
        if ( d < 0 ) // move right
        {
            d = d + 4*x + 6;
        }
        else // move down 
        {
            d = d + 4*(x - y) + 10;
            //Since we've started at the top of the circle
            y = y - 1;
        }
         
        // Since we have started at top of the circle
        x = x + 1;                                
    }        
}

void draw_line_chat_gpt(unsigned char* pixels, unsigned int height, unsigned int width, unsigned int x1, unsigned int y1, unsigned int x2, unsigned int y2, unsigned char color)
{
    int dx = x2 - x1;
    int dy = y2 - y1;
    int steps, k;
    float x_inc, y_inc, x, y;

    if (abs(dx) > abs(dy))
    {
        steps = abs(dx);
    }
    else
    {
        steps = abs(dy);
    }

    x_inc = (float)dx / (float)steps;
    y_inc = (float)dy / (float)steps;

    x = x1;
    y = y1;

    for (k = 0; k < steps; k++)
    {
        int px = (int)(x + 0.5);
        int py = (int)(y + 0.5);
        int index = py * width + px;
        pixels[index] = color;
        x += x_inc;
        y += y_inc;
    }
}

void draw_line_by_three(unsigned char* pixels, unsigned int height, unsigned int width, unsigned int x1, unsigned int y1, unsigned int x2, unsigned int y2, unsigned char color)
{
    /*
    x1 = x1*(-1);
    x2 = x2*(-1);
     */

    x1 = x1*3;
    x2 = x2*3;

    // Steep is Vertically inclined
    unsigned char steep = (abs(y2 - y1) > abs(x2 - x1)) ? 1 : 0;
    // d is for direction
    int d, ystep;
    /*unsigned*/ int tmp, dx, dy, x, y;

    // When line is steep(vertically inclined), make it horizontally inclined(less steep)    
    if (steep == 1)
    {        
        // swap(x1, y1)
        tmp = y1;
        y1 = x1;
        x1 = tmp;

        // swap(x2, y2)
        tmp = y2;
        y2 = x2;
	    x2 = tmp;  
    }
    
    //We always move from left to right(that is x is always incremented). For that x1 has to be closer to origin on x-axis than x2
    if ( x1 > x2 ) 
    {	  
	    //swap(x1, x2);	
	    tmp = x2;
	    x2 = x1;
	    x1 = tmp;
		
	    //swap(y1, y2)	
	    tmp = y2;
	    y2 = y1;
	    y1 = tmp;
    }

    // Change in x and change in y
    dx = x2 - x1;	  
    dy = abs(y2 - y1);

    //Initial value, the first and the last points are always on the line, so error is zero(2e=2(0)=0)
    //e = dyX - dxY + c
    //eR = dy(X + 1) - dxY + c = e + dy
    //eD = dy(X + 1) - dx(Y + 1) + c = e + dy - dx
    //d = eR + eD
    d = 2*dy - dx;

    if ( y1 < y2 ) 
    {
	    ystep = 1;
    }
    else
    {
        ystep = -1;
    }

    //Initial values(initial ordinate pair)		 
    x = x1;
    y = y1;

    while ( x <= x2 ) 
    {
        //x is reflected as y(transitive) 		 
	    if ( steep == 1 )
        {
	        //cxt.fillRect(y, x, 1, 1);
	        //setPixelColor(cxt, y, x, color);
            pixels[x*width + (/*(width - 1) -*/ y)] = color;
            pixels[x*width + (/*(width - 1) -*/ y + 1)] = color;
            pixels[x*width + (/*(width - 1) -*/ y + 2)] = color;
	    }  
	    else
        {
            //cxt.fillRect(x, y, 1, 1);	
            //setPixelColor(cxt, x, y, color); 	  
            pixels[y*width + (/*(width - 1) -*/ x)] = color;
            pixels[y*width + (/*(width - 1) -*/ x + 1)] = color;
            pixels[y*width + (/*(width - 1) -*/ x + 2)] = color;
	    }  

        //We only allow two moves, move to the right, or move diagonally(move to the right and downwards). when we move to the right we only increment x otherwise we increment both(sign of ystep)
	    if ( d < 0 ) 
        {   
            d = d + 2*dy;
        }
        else 
        {		    
	        d = d + 2*dy - 2*dx;
            y = y + ystep;			
        }  

        x = x + 3; 	    
    }    

    /*
    for (unsigned int i = 0; i < height; i++)
    {
        unsigned char* line = malloc(width*(sizeof(unsigned char)));

        for (unsigned int j = 0; j < width; j++)
        {
            line[j] = pixels[i*width + ((width - 1) - j)];
        }

        for (unsigned int j = 0; j < width; j++)
        {
            pixels[i*width + j ] = line[j];
        }

        free(line);
    }
     */
}



 /*
    FOR DOCUMENTATION PURPOSES, WHEN LINES ARE 720 AND COLUMNS ARE 1424
    -------------------------------------------------------------------
    // ////////////////////////////////////////////////////////////////////////////
    //  Horizontal line and a vertical line. All other lines are between two lines
    // ////////////////////////////////////////////////////////////////////////////
  */
 /*
    //unsafe { draw_line(self.idat.data.as_mut_ptr(), self.height.try_into().unwrap(), self.width.try_into().unwrap(), 205, 200, 2000, 200, 1) };
    //unsafe { draw_line(self.idat.data.as_mut_ptr(), self.height.try_into().unwrap(), self.width.try_into().unwrap(), 500, 200, 1000, 700, 1) };
  */
void draw_line(unsigned char* pixels, unsigned int height, unsigned int width, unsigned int x1, unsigned int y1, unsigned int x2, unsigned int y2, unsigned char color)
{
    /*
    x1 = x1*(-1);
    x2 = x2*(-1);
     */
    // Steep is Vertically inclined
    unsigned char steep = (abs(y2 - y1) > abs(x2 - x1)) ? 1 : 0;
    // d is for direction. The directions are two, move to right, or move to right and downwards
    int d, ystep;
    /*unsigned*/ int tmp, dx, dy, x, y;

    // When line is steep(vertically inclined), make it horizontally inclined(less steep)    
    if (steep == 1)
    {        
        // swap(x1, y1)
        tmp = y1;
        y1 = x1;
        x1 = tmp;

        // swap(x2, y2)
        tmp = y2;
        y2 = x2;
	    x2 = tmp;  
    }
    
    //We always move from left to right(that is x is always incremented). For that x1 has to be closer to origin on x-axis than x2
    if ( x1 > x2 ) 
    {	  
	    //swap(x1, x2);	
	    tmp = x2;
	    x2 = x1;
	    x1 = tmp;
		
	    //swap(y1, y2)	
	    tmp = y2;
	    y2 = y1;
	    y1 = tmp;
    }

    // Change in x and change in y. Change in x cant be negative because we aready made sure that we would always move from left to right
    dx = x2 - x1; 	  
    dy = abs(y2 - y1);

    //Initial value, the first and the last points are always on the line, so error is zero(2e=2(0)=0)
    //e = dyX - dxY + c
    //eR = dy(X + 1) - dxY + c = e + dy
    //eD = dy(X + 1) - dx(Y + 1) + c = e + dy - dx
    //d = eR + eD
    //d = e + dy + e + dy - dx
    //d = 2e + 2dy - dx
    // The first and the last points are always on the line, so error is zero(2e=2(0)=0) hence...     
    d = 2*dy - dx;

    if ( y1 < y2 ) 
    {
	    ystep = 1;
    }
    else
    {
        ystep = -1;
    }

    //Initial values(initial ordinate pair)		 
    x = x1;
    y = y1;

    while ( x <= x2 ) 
    {
        //x is reflected as y(transitive) 		 
	    if ( steep == 1 )
        {
	        //cxt.fillRect(y, x, 1, 1);
	        //setPixelColor(cxt, y, x, color);
            pixels[x*width + (/*(width - 1) -*/ y)] = color;
	    }  
	    else
        {
            //cxt.fillRect(x, y, 1, 1);	
            //setPixelColor(cxt, x, y, color); 	  
            pixels[y*width + (/*(width - 1) -*/ x)] = color;
	    }  

        //We only allow two moves, move to the right, or move diagonally(move to the right and downwards). when we move to the right we only increment x otherwise we increment both(sign of ystep)
	    if ( d < 0 ) 
        {   
            d = d + 2*dy;
        }
        else 
        {		    
	        d = d + 2*dy - 2*dx;
            y = y + ystep;			
        }  

        x = x + 1; 	    
    }    

    /*
    for (unsigned int i = 0; i < height; i++)
    {
        unsigned char* line = malloc(width*(sizeof(unsigned char)));

        for (unsigned int j = 0; j < width; j++)
        {
            line[j] = pixels[i*width + ((width - 1) - j)];
        }

        for (unsigned int j = 0; j < width; j++)
        {
            pixels[i*width + j ] = line[j];
        }

        free(line);
    }
     */
}


/* ****************************************************************************************** */
/* ******************************************* CRC ****************************************** */
/* ****************************************************************************************** */  
/* Table of CRCs of all 8-bit messages. */
unsigned long crc_table[256];   
/* Flag: has the table been computed? Initially false. */
int crc_table_computed = 0;

/* Make the table for a fast CRC. */
void make_crc_table(void)
{

    unsigned long c;
    int n, k;
   
    for (n = 0; n < 256; n++) {

        c = (unsigned long) n;

        //printf(" c =  ");

        for (k = 0; k < 8; k++) {

            if (c & 1) {

                c = 0xedb88320L ^ (c >> 1);

            } else {

                c = c >> 1;
            }        
        }
    
        crc_table[n] = c;
    }

    crc_table_computed = 1;
}

/* Update a running CRC with the bytes buf[0..len-1]--the CRC
   should be initialized to all 1's, and the transmitted value
   is the 1's complement of the final running CRC (see the
   crc() routine below)). */   
unsigned long update_crc(unsigned long crc, unsigned char *buf, unsigned int len)
{
    unsigned long c = crc;
    int n;
   
    if (!crc_table_computed) {

       make_crc_table();
    }

    for (n = 0; n < len; n++) {

       c = crc_table[(c ^ buf[n]) & 0xff] ^ (c >> 8);
    }
    return c;
}
/* ****************************************************************************************** */
/* ***************************************** CRC Ends *************************************** */
/* ****************************************************************************************** */



/* ****************************************************************************************** */
/* ***************************************** Endiness *************************************** */
/* ****************************************************************************************** */
unsigned int little_endian_read_u32(unsigned char* ptr)
{

    return 0;
}

unsigned char* little_endian_write_u32(unsigned char* ptr, unsigned int value)
{

    return NULL;
} 

/*
    @ptr has an array of size 4, this array has a big-endian representation of a value.
    This function will turn the big-endian representation to it equivalent value in little-endian
 */
unsigned int big_endian_read_u32(unsigned char* ptr)
{
    
    return (ptr[0] << 24 | ptr[1] << 16 | ptr[2] << 8 | ptr[3]);
}

/*
    @ptr has array of size 4, value is stored in this array in its big-endian form/representation
 */
void big_endian_write_u32(unsigned char* ptr, unsigned int value)
{  

    ptr[0] = (value & 0xff000000) >> 24;
    ptr[1] = (value & 0x00ff0000) >> 16;
    ptr[2] = (value & 0x0000ff00) >> 8;
    ptr[3] = (value & 0x000000ff);    
}
/* ****************************************************************************************** */
/* **************************************** Endiness Ends *********************************** */
/* ****************************************************************************************** */

/* ****************************************************************************************** */
/* *************************************** File Read Write ********************************** */
/* ****************************************************************************************** */

void write_png (const char* name, const unsigned char* ptr, unsigned int len) 
{
    /*
    printf("----> FILE NAME = %s\n", name);

    for (unsigned int i = 0; i < len; i++)
    {
        printf("%.2x ", ptr[i]);
    }
     */

    FILE* file = fopen(name, "wb");    

    fwrite(ptr, len, 1, file);

    fclose(file);
}

/* ****************************************************************************************** */
/* ************************************ File Read Write Ends ******************************** */
/* ****************************************************************************************** */



/*
unsigned int image(int num) {
    
    return 0;
}
 */
