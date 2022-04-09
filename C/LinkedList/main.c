#include <stdio.h>
#include <stdlib.h>
#include <locale.h>

typedef struct Node *Node;

struct Node
{
    int Number;
    Node Next;
};

Node New(int number);
void PrintLinkedList(Node head);
void Add(Node head, Node newNode);
void Next(Node *node);

void main()
{
    Node head = New(5);
    Add(head, New(10));
    Add(head, New(15));
    Add(head, New(20));

    PrintLinkedList(head);
    getchar();
}

Node New(int number)
{
    Node node = malloc(sizeof(struct Node));
    node->Number = number;
    node->Next = NULL;
}
void Add(Node head, Node newNode)
{
    while (head->Next != NULL)
    {
        Next(&head);
    }
    head->Next = newNode;
}
void Next(Node *node)
{
    (*node) = (*node)->Next;
}
void PrintLinkedList(Node head)
{
    int i = 0;
    while (head != NULL)
    {
        i++;
        printf("Node[%d] = %d\n", i, head->Number);
        Next(&head);
    }
}