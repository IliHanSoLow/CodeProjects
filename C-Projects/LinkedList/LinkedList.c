#include <stdio.h>

struct LinkedList *root;

struct node {
  int age;
  char name[100];
  struct LinkedList *next;
};

void setRoot(struct node rt) { *root = rt; }

void generateListElement(int age_, char name_[100]) {}

int main() { return 0; }
