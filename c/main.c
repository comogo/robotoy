#include <SDL2/SDL.h>
#include <signal.h>

#define JOYSTICK_DEADZONE 4000
#define JOYSTICK_BUTTON_A 0
#define JOYSTICK_BUTTON_B 1
#define JOYSTICK_BUTTON_X 2
#define JOYSTICK_BUTTON_Y 3
#define JOYSTICK_BUTTON_SELECT 4
#define JOYSTICK_BUTTON_START 6
#define JOYSTICK_BUTTON_L3 7
#define JOYSTICK_BUTTON_R3 8
#define JOYSTICK_BUTTON_LB 9
#define JOYSTICK_BUTTON_RB 10
#define JOYSTICK_BUTTON_UP 11
#define JOYSTICK_BUTTON_DOWN 12
#define JOYSTICK_BUTTON_LEFT 13
#define JOYSTICK_BUTTON_RIGHT 14
#define JOYSTICK_AXIS_LEFT_HORIZONTAL 0
#define JOYSTICK_AXIS_LEFT_VERTICAL 1
#define JOYSTICK_AXIS_RIGHT_HORIZONTAL 2
#define JOYSTICK_AXIS_RIGHT_VERTICAL 3
#define JOYSTICK_AXIS_TRIGGER_LEFT 4
#define JOYSTICK_AXIS_TRIGGER_RIGHT 5

typedef struct
{
  u_int8_t a;
  u_int8_t b;
  u_int8_t x;
  u_int8_t y;
  u_int8_t select;
  u_int8_t start;
  u_int8_t l1;
  u_int8_t r1;
  int16_t l2;
  int16_t r2;
  int16_t yaw;
  int16_t throtle;
  int16_t roll;
  int16_t pitch;
} ControllerState;

ControllerState controller_state;
int break_loop = 0;
SDL_Event event;
SDL_Joystick *joystick;

void handle_sigint(int sig);
void quit();
void initialize();
void loop(SDL_Joystick *joystick);
void update_axis(int axis, int value);
void update_button(int button, int value);
int16_t calculate_with_deadzone(int value);
void print_controller_state();

int main(int argc, char *argv[])
{
  initialize();
  signal(SIGINT, handle_sigint);
  printf("initialized\n");
  loop(joystick);
  quit();
  return 0;
}

void handle_sigint(int sig)
{
  break_loop = 1;
  printf("INT\n");
}

void quit()
{
  if (SDL_JoystickGetAttached(joystick))
  {
    SDL_JoystickClose(joystick);
    printf("joystick detached\n");
  }

  SDL_Quit();
  printf("Bye!\n");
}

void initialize()
{
  if (SDL_Init(SDL_INIT_JOYSTICK) < 0)
  {
    fprintf(stderr, "Cound't initialize SDL: %s\n", SDL_GetError());
    exit(1);
  }

  controller_state.l2 = -32768;
  controller_state.r2 = -32768;
}

void loop(SDL_Joystick *joystick)
{
  while (break_loop == 0)
  {
    while (SDL_PollEvent(&event))
    {
      switch (event.type)
      {
      case SDL_QUIT:
        break_loop = 1;
        break;
      case SDL_JOYAXISMOTION:
        update_axis(event.jaxis.axis, event.jaxis.value);
        break;
      case SDL_JOYBUTTONDOWN:
        update_button(event.jbutton.button, 1);
        if (event.jbutton.button == JOYSTICK_BUTTON_UP)
        {
          print_controller_state();
        }
        break;
      case SDL_JOYBUTTONUP:
        update_button(event.jbutton.button, 0);
        break;
      case SDL_JOYDEVICEADDED:
        joystick = SDL_JoystickOpen(event.jdevice.which);
        printf("joytick attached: %s\n", SDL_JoystickName(joystick));
        break;
      case SDL_JOYDEVICEREMOVED:
        SDL_JoystickClose(joystick);
        printf("joystick detached\n");
        break;
      }
    }
  }
}

void update_axis(int axis, int value)
{
  switch (axis)
  {
  case JOYSTICK_AXIS_LEFT_HORIZONTAL:
    controller_state.yaw = calculate_with_deadzone(value);
    break;
  case JOYSTICK_AXIS_LEFT_VERTICAL:
    controller_state.throtle = calculate_with_deadzone(value);
    break;
  case JOYSTICK_AXIS_RIGHT_HORIZONTAL:
    controller_state.roll = calculate_with_deadzone(value);
    break;
  case JOYSTICK_AXIS_RIGHT_VERTICAL:
    controller_state.pitch = calculate_with_deadzone(value);
    break;
  case JOYSTICK_AXIS_TRIGGER_LEFT:
    controller_state.l2 = value;
    break;
  case JOYSTICK_AXIS_TRIGGER_RIGHT:
    controller_state.r2 = value;
    break;
  }
}

void update_button(int button, int value)
{
  switch (button)
  {
  case JOYSTICK_BUTTON_A:
    controller_state.a = value;
    break;
  case JOYSTICK_BUTTON_B:
    controller_state.b = value;
    break;
  case JOYSTICK_BUTTON_X:
    controller_state.x = value;
    break;
  case JOYSTICK_BUTTON_Y:
    controller_state.y = value;
    break;
  case JOYSTICK_BUTTON_SELECT:
    controller_state.select = value;
    break;
  case JOYSTICK_BUTTON_START:
    controller_state.start = value;
    break;
  }
}

void print_controller_state()
{
  printf("{\n");
  printf("  a: %i,\n", controller_state.a);
  printf("  b: %i,\n", controller_state.b);
  printf("  x: %i,\n", controller_state.x);
  printf("  y: %i,\n", controller_state.y);
  printf("  select: %i,\n", controller_state.select);
  printf("  start: %i,\n", controller_state.start);
  printf("  l1: %i,\n", controller_state.l1);
  printf("  r1: %i,\n", controller_state.r1);
  printf("  l2: %i,\n", controller_state.l2);
  printf("  r2: %i,\n", controller_state.r2);
  printf("  yaw: %i,\n", controller_state.yaw);
  printf("  throtle: %i,\n", controller_state.throtle);
  printf("  roll: %i,\n", controller_state.roll);
  printf("  pitch: %i,\n", controller_state.pitch);
  printf("}\n");
}

int16_t calculate_with_deadzone(int value)
{
  if (value > -JOYSTICK_DEADZONE && value < JOYSTICK_DEADZONE)
  {
    return 0;
  }
  else
  {
    return value;
  }
}
