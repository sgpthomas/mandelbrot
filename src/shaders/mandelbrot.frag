#version 410 core

out vec4 color;

// uniforms
uniform dvec2 start_z;
uniform int max_iter;
uniform vec2 screen_size;
uniform float zoom;
uniform dvec2 center;
uniform int mode;
uniform int color_mode;

vec4 colormap_1(float x);
vec4 colormap_ocean(float x);

void main() {

  float zoom_factor = 2 / zoom;
  float max_screen_size = max(screen_size.x, screen_size.y);

  double offset_x = 0.0;
  double offset_y = 0.0;
  if (screen_size.x == max_screen_size) {
    offset_y = (screen_size.x - screen_size.y) / 2;
  } else {
    offset_x = (screen_size.y - screen_size.x) / 2;
  }
  double x = ((gl_FragCoord.x + offset_x) / max_screen_size)  * (2 * zoom_factor) + center.x - zoom_factor;
  double y = ((gl_FragCoord.y + offset_y) / max_screen_size) * (2 * zoom_factor) + center.y - zoom_factor;

  dvec2 coord = dvec2(x, y);
  float i;
  dvec2 z;
  if (mode == 0) { // mandelbrot
    dvec2 c = coord;
    z = start_z;

    double zx_square = z.x * z.x;
    double zy_square = z.y * z.y;

    int p = 0;
    int ptot = 2;
    dvec2 period;
    for (i = 0; i < max_iter; i++) {
      if (p == ptot) {
        ptot += ptot;
        period = z;
      }
      p++;
      z.y = z.x * z.y;
      z.y += z.y; // zi * 2
      z.x = zx_square - zy_square;
      z += c;
      zx_square = z.x * z.x;
      zy_square = z.y * z.y;
      if (zx_square + zy_square > 4.0) {
        break;
      }

      if (period == z) {
        i = max_iter;
        break;
      }
    }
  // float i;
  // vec2 z;
  // if (mode == 0) { // mandelbrot
  //   vec2 c = coord;
  //   z = start_z;

  //   float zx_square = z.x * z.x;
  //   float zy_square = z.y * z.y;

  //   int p = 0;
  //   int ptot = 2;
  //   vec2 period;
  //   for (i = 0; i < max_iter; i++) {
  //     if (p == ptot) {
  //       ptot += ptot;
  //       period = z;
  //     }
  //     p++;
  //     z.y = z.x * z.y;
  //     z.y += z.y; // zi * 2
  //     z.x = zx_square - zy_square;
  //     z += c;
  //     zx_square = z.x * z.x;
  //     zy_square = z.y * z.y;
  //     if (zx_square + zy_square > 4.0) {
  //       break;
  //     }

  //     if (period == z) {
  //       i = max_iter;
  //       break;
  //     }
  //   }
  } else { // julia
    dvec2 c = start_z;
    z = coord;

    double zx_square = z.x * z.x;
    double zy_square = z.y * z.y;
    for (i = 0; i < max_iter; i++) {
      z.y = z.x * z.y;
      z.y += z.y; // zi * 2
      z.x = zx_square - zy_square;
      z += c;
      zx_square = z.x * z.x;
      zy_square = z.y * z.y;
      if(length(z) > 2.0) {
        break;
      }
    }
  }

  if (i == max_iter) {
    color = vec4(0.0, 0.0, 0.0, 1.0);
  } else {
    float val = sin(0.016 * (i - log(log(float(length(z))) / log(2.0))));
    if (color_mode == 0) {
      color = colormap_1(val);
    } else if (color_mode == 1) {
      color = colormap_ocean(val);
    }
  }
}

// color map
float colormap_red_1(float x) {
	if (x < 0.0906416957946237) {
		return -1.48766695652174E+03 * x + 1.65896666666667E+02;
	} else if (x < 0.181137969063194) {
		return 1.62263833992095E+03 * x - 1.16026679841898E+02;
	} else if (x < 0.2716806139960391) {
		return -1.40227075098820E+03 * x + 4.31899209486178E+02;
	} else if (x < 0.3621693275308975) {
		return 2.21145652173927E+03 * x - 5.49880434782653E+02;
	} else if (x < 0.4514347780510689) {
		return -2.73075098814252E+02 * x + 3.49940711462467E+02;
	} else if (x < 0.5478389816716595) {
		return 2.75424347826088E+02 * x + 1.02328985507251E+02;
	} else if (x < 0.6384253260915684) {
		return 1.95770750987722E+01 * x + 2.42492094861655E+02;
	} else if (x < 0.7280391465804739) {
		return -5.92081027667844E+02 * x + 6.32990118576982E+02;
	} else if (x < 0.8191050219893012) {
		return -1.05189130434770E+03 * x + 9.67749999999916E+02;
	} else if (x < 0.9092300295745469) {
		return 1.64974505928811E+03 * x - 1.24517391304309E+03;
	} else {
		return -8.20731225296366E+02 * x + 1.00105731225287E+03;
	}
}

float colormap_green_1(float x) {
	if (x < 0.09069203671589791) {
		return -9.49076521739127E+02 * x + 2.05970000000000E+02;
	} else if (x < 0.1811205395903491) {
		return 1.14400395256917E+03 * x + 1.61442687747026E+01;
	} else if (x < 0.271076794014141) {
		return -7.04272727272755E+02 * x + 3.50905138339931E+02;
	} else if (x < 0.3618506954718166) {
		return -6.35000000000221E+01 * x + 1.77206521739141E+02;
	} else if (x < 0.4527247821743651) {
		return -1.40603557312254E+03 * x + 6.63003952569178E+02;
	} else if (x < 0.5472660653935183) {
		return 1.73713913043494E+03 * x - 7.59989130434857E+02;
	} else if (x < 0.6379975539161487) {
		return -7.00507905138483E+02 * x + 5.74052371541584E+02;
	} else if (x < 0.7283304980067641) {
		return 5.64723320158019E+02 * x - 2.33162055335916E+02;	} else if (x < 0.8189077039268755) {
		return -1.29283992094844E+03 * x + 1.11975790513821E+03;	} else if (x < 0.9094178747563795) {
		return 2.14293675889271E+03 * x - 1.69382608695601E+03;
	} else {
		return -1.75290118577070E+03 * x + 1.84911857707505E+03;
	}
}

float colormap_blue_1(float x) {
	if (x < 0.1835817221386023) {
		return -4.93278367346940E+02 * x + 2.25853877551021E+02;
	} else if (x < 0.2718482976477959) {
		return -1.04124223602495E+03 * x + 3.26450028232661E+02;
	} else if (x < 0.3623519200472859) {
		return 1.21151976284592E+03 * x - 2.85959486166031E+02;
	} else if (x < 0.4526344257525674) {
		return -1.38645849802374E+03 * x + 6.55422924901199E+02;
	} else if (x < 0.5474992417588231) {
		return 8.80275652173975E+02 * x - 3.70578985507278E+02;
	} else if (x < 0.6375259518892261) {
		return -1.24038339920972E+03 * x + 7.90480237154278E+02;
	} else if (x < 0.7280438873117513) {
		return 2.36255138339872E+03 * x - 1.50648418972297E+03;
	} else if (x < 0.8192397843702398) {
		return -6.51816205533491E+02 * x + 6.88107707509788E+02;
	} else if (x < 0.9092328860678134) {
		return -1.35533596837590E+01 * x + 1.65217391304318E+02;
	} else {
		return -1.19420158102770E+03 * x + 1.23870158102770E+03;
	}
}

vec4 colormap_1(float x) {
	float r = clamp(colormap_red_1(x) / 255.0, 0.0, 1.0);
	float g = clamp(colormap_green_1(x) / 255.0, 0.0, 1.0);
	float b = clamp(colormap_blue_1(x) / 255.0, 0.0, 1.0);
	return vec4(r, g, b, 1.0);
}

float colormap_red_ocean(float x) {
	if (x < 0.84121424085) {
		const float pi = 3.141592653589793238462643383279502884197169399;
		const float a = 92.39421034238549;
		const float b = 88.02925388837211;
		const float c = 0.5467741159150409;
		const float d = 0.03040219113949284;
		return a * sin(2.0 * pi / c * (x - d)) + b;
	} else {
		return 105.0;
	}
}

float colormap_green_ocean(float x) {
	if (x < 0.84121424085) {
		const float pi = 3.141592653589793238462643383279502884197169399;
		const float a = 92.44399971120093;
		const float b = 22.7616696017667;
		const float c = 0.3971750420482239;
		const float d = 0.1428144080827581;
		const float e = 203.7220396611977;
		const float f = 49.51517183258432;
		float v = (a * x + b) * sin(2.0 * pi / c * (x + d)) + (e * x + f);
		if (v > 255.0) {
			return 255.0 - (v - 255.0);
		} else {
			return v;
		}
	} else {
		return 246.0;
	}
}

float colormap_blue_ocean(float x) {
	if (x < 0.84121424085) {
		const float pi = 3.141592653589793238462643383279502884197169399;
		const float a = 251.0868719483008;
		const float b = 0.5472498585835275;
		const float c = 0.02985857858149428;
		const float d = 225.9495771701237;
		float v = a * sin(2.0 * pi / b * (x - c)) + d;
		if (v > 255.0) {
			return 255.0 - (v - 255.0);
		} else if (v < 0.0) {
			return -v;
		} else {
			return v;
		}
	} else {
		return 234.0;
	}
}

// R1 - 105 = 0
// => 0.8344881408181015

// B1 - 234 = 0
// => 0.847940340889657

vec4 colormap_ocean(float x) {
    float r = clamp(colormap_red_ocean(x) / 255.0, 0.0, 1.0);
    float g = clamp(colormap_green_ocean(x) / 255.0, 0.0, 1.0);
    float b = clamp(colormap_blue_1(x) / 255.0, 0.0, 1.0);
    return vec4(r, g, b, 1.0);
}
