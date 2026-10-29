// https://pythontutor.com/render.html

#include <stdlib.h>  // For malloc and free
#include <stdio.h>   // For printf
#include <string.h>  // For strcpy

typedef struct {
  int number;
  char text[20];
} Data;

int fun_b(int l, Data* data_ptr) {
    l = l + 1;
    data_ptr->number = l;
    return data_ptr->number;
}

int fun_a(int j) {
  int k = 20;

  // Allocate memory on the heap for the struct
  Data *data_ptr = (Data *)malloc(sizeof(Data));

  // Initialize allocated memory
  data_ptr->number = k;
  strcpy(data_ptr->text, "Hello heap!");

  int r = fun_b(j, data_ptr);

  // Free the allocated memory
  free(data_ptr);
  return r;
}

int main() {
  int i = 5;
  fun_a(i);
  return 0;
}