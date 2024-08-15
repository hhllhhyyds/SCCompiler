struct Point
{
    int x;
    int y;
};

int main() 
{
    int arr[10];
    int i;
    struct Point pt;
    pt.x = 1024;
    pt.y = 768;
    for (i = 0; i < 10; i = i + 1)
    {
        arr[i] = i;
        if (i == 6)
        {
            continue;
        }
        else
        {
            printf("arr[%d] = %d", i, arr[i]);
        }
    }
    printf("pt.x = %d, pt.y = %d", pt.x, pt.y);
}