int main() {
  int a = 1;
  {
    a = a + 2;
    int a = 3;
    a = a + 4;
    {
      int a;
      {
        int a;
        {
          {
            const int x = 7;
            {
              {
                int x = 5;

                {
                  {
                    int x;
                    int x;
                    x + 7;
                    ;
                  }
                  int y = 5;
                  int x = 7;
                  const int x = 8;
                  {
                    int x;
                    // return x;
                  }
                  {
                    // int x;
                    return x;
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  a = a + 5;
  {
    int a;
    return a;
  }
  return a;
}
