
// pure rust implementation of a bpm analysis algorithm
// derived/copied from Mark Hill's implementation, available
// at http://www.pogo.org.uk/~mark/bpm-tools/ contact 
// mark@xwax.org for more information.

use audio_in::AudioBuffer;

// #define RATE 44100 /* of input data */
// #define LOWER 84.0
// #define UPPER 146.0

// #define BLOCK 4096
// #define INTERVAL 128

// #define ARRAY_SIZE(x) (sizeof(x) / sizeof(*(x)))

/*
 * Sample from the metered energy
 *
 * No need to interpolate and it makes a tiny amount of difference; we
 * take a random sample of samples, any errors are averaged out.
 */

/*
static double sample(float nrg[], size_t len, double offset)
{
    double n;
    size_t i;

    n = floor(offset);
    i = (size_t)n;

    return (n >= 0.0 && n < (double)len) ? nrg[i] : 0.0;
}
*/

/*
 * Test an autodifference for the given interval
 */

/* 
double autodifference(float nrg[], size_t len, double interval)
{
    size_t n;
    double mid, v, diff, total;
    static const double beats[] = { -32, -16, -8, -4, -2, -1,
                    1, 2, 4, 8, 16, 32 },
            nobeats[] = { -0.5, -0.25, 0.25, 0.5 };

    mid = drand48() * len;
    v = sample(nrg, len, mid);

    diff = 0.0;
    total = 0.0;

    for (n = 0; n < ARRAY_SIZE(beats); n++) {
        double y, w;

        y = sample(nrg, len, mid + beats[n] * interval);

        w = 1.0 / fabs(beats[n]);
        diff += w * fabs(y - v);
        total += w;
    }

    for (n = 0; n < ARRAY_SIZE(nobeats); n++) {
        double y, w;

        y = sample(nrg, len, mid + nobeats[n] * interval);

        w = fabs(nobeats[n]);
        diff -= w * fabs(y - v);
        total += w;
    }

    return diff / total;
}
*/

/*
 * Beats-per-minute to a sampling interval in energy space
 */

/*
double bpm_to_interval(double bpm)
{
    double beats_per_second, samples_per_beat;

    beats_per_second = bpm / 60;
    samples_per_beat = RATE / beats_per_second;
    return samples_per_beat / INTERVAL;
}
*/

/*
 * Sampling interval in enery space to beats-per-minute
 */
/*
double interval_to_bpm(double interval)
{
    double samples_per_beat, beats_per_second;

    samples_per_beat = interval * INTERVAL;
    beats_per_second = (double)RATE / samples_per_beat;
    return beats_per_second * 60;
}
*/

/*
 * Scan a range of BPM values for the one with the
 * minimum autodifference
 */

/*
double scan_for_bpm(float nrg[], size_t len,
        double slowest, double fastest,
        unsigned int steps,
        unsigned int samples,
        FILE *file)
{
    double step, interval, trough, height;
    unsigned int s;

    slowest = bpm_to_interval(slowest);
    fastest = bpm_to_interval(fastest);
    step = (slowest - fastest) / steps;

    height = INFINITY;
    trough = NAN;

    for (interval = fastest; interval <= slowest; interval += step) {
        double t;

        t = 0.0;
        for (s = 0; s < samples; s++)
            t += autodifference(nrg, len, interval);

        if (file != NULL) {
            fprintf(file, "%lf\t%lf\n",
                interval_to_bpm(interval),
                t / samples);
        }

        /* Track the lowest value */

        if (t < height) {
            trough = interval;
            height = t;
        }
    }

    return interval_to_bpm(trough);
}
*/

// main scanning function
/*
int main(int argc, char *argv[])
{
    float *nrg = NULL;
    size_t len = 0, buf = 0;
    off_t n = 0;
    double bpm, min = LOWER, max = UPPER, v = 0.0;
    const char *format = "%0.3f";
    FILE *fdiff = NULL, *fnrg = NULL;

    argv += optind;
    argc -= optind;

    if (argc > 0) {
        fprintf(stderr, "%s: Too many arguments\n", NAME);
        return EX_USAGE;
    }

    for (;;) {
        float z;

        if (fread(&z, sizeof z, 1, stdin) != 1)
            break;

        /* Maintain an energy meter (similar to PPM) */

        z = fabsf(z);
        if (z > v) {
            v += (z - v) / 8;
        } else {
            v -= (v - z) / 512;
        }

        /* At regular intervals, sample the energy to give a
         * low-resolution overview of the track */

        n++;
        if (n != INTERVAL)
            continue;

        n = 0;

        if (len == buf) {
            size_t n;

            n = buf + BLOCK;
            nrg = realloc(nrg, n * sizeof(*nrg));
            if (nrg == NULL) {
                perror("realloc");
                return -1;
            }
            buf = n;
        }

        if (fnrg != NULL) {
            fprintf(fnrg, "%lf\t%lf\n",
                (double)len * INTERVAL / RATE, v);
        }

        nrg[len++] = v;
    }

    bpm = scan_for_bpm(nrg, len, min, max, 1024, 1024, fdiff);

    printf(format, bpm);
    putc('\n', stdout);


    return 0;
} */