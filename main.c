#include <SDL2/SDL.h>
#include <signal.h>


#define JOYSTICK_DEADZONE 8000
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

int break_loop = 0;
SDL_Event event;
SDL_Joystick *joystick;

void handle_sigint(int sig);
void quit();
void initialize();
void loop(SDL_Joystick *joystick, SDL_Event *event, int *status);

int main(int argc, char *argv[])
{
	initialize();
	signal(SIGINT, handle_sigint);
	loop(joystick, &event, &break_loop);
	quit();
	return 0;
}

void handle_sigint(int sig)
{
	break_loop = 1;
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

void initialize() {
	if (SDL_Init(SDL_INIT_JOYSTICK) < 0)
	{
		fprintf(stderr, "Cound't initialize SDL: %s\n", SDL_GetError());
		exit(1);
	}
}

void loop(SDL_Joystick *joystick, SDL_Event *event, int *status) {
	while(status == 0)
	{
		SDL_WaitEvent(event);

		switch(event.type)
		{
			case SDL_QUIT:
				status = 1;
				break;
			case SDL_JOYAXISMOTION:
				if (event.jaxis.value < -JOYSTICK_DEADZONE || event.jaxis.value > JOYSTICK_DEADZONE)
				{
					printf("axis: %i %i\n", event.jaxis.axis, event.jaxis.value);
				}
				break;
			case SDL_JOYBUTTONDOWN:
				printf("button: %i\n", event.jbutton.button);
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
