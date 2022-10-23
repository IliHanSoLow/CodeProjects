#include <stdio.h>

void push();
int pop();
void printList();

int queue[10];
int count = 0;
int queuelength = 10;

int main()
{
    printf("QueueLength: %i\n", queuelength);
    /*queue=(int *)malloc(10*sizeof(int));*/
    push(1);
    push(2);
    push(3);
    push(4);
    push(5);
    push(5);
    push(6);
    push(7);
    push(8);
    push(9);
    push(10);
    push(11);
    push(12);
    push(13);

    printList();

    pop();

    printList();

    return 0;
}

void printList()
{
    printf("List: ");
    for (int i = 0; i < count; i++)
    {
        printf("%i; ", queue[i]);
    }
    printf("\nLength :%i\nQueueLength: %i\n", count, queuelength);
}

void push(int a)
{
    if (count >= queuelength)
    {
        int * queue_;
        queue_ = (int *)malloc((queuelength + 10) * sizeof(int));
        for (int i = 0; i < count; i++)
        {
            queue_[i] = queue[i];
        }
        queue_ = queue;
        queuelength += 10;
    }

    if (count < 0)
    {
        printf("count<0");
        return;
    }
    queue[count] = a;
    count++;
}

int pop()
{
    if (count < 0)
    {
        return;
    }

    int tmp;
    tmp = queue[0];
    for (int i = 0; i < count; i++)
    {
        queue[i] = queue[i + 1];
    }
    count--;
    return tmp;
}