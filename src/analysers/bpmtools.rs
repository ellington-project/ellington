// pure rust implementation of a bpm analysis algorithm
// derived/copied from Mark Hill's implementation, available
// at http://www.pogo.org.uk/~mark/bpm-tools/ contact
// mark@xwax.org for more information.

use rand::distributions::Uniform;
use rand::ThreadRng;
use rand::{thread_rng, Rng};
use std::f32;

#[derive(Debug)]
pub struct BpmTools {
    // tunable parameters for the algorithm.
    pub lower: f32,
    pub upper: f32,
    pub interval: u64,
    pub rate: f32,
    pub steps: u32,
    pub samples: u32,
    rng: ThreadRng,
}

impl BpmTools {
    pub fn default() -> BpmTools {
        BpmTools {
            lower: 50.0,
            upper: 450.0,
            interval: 64,
            rate: 44100.0,
            steps: 1024,
            samples: 1024,
            rng: thread_rng(),
        }
    }

    /*
     * main analysis function
     * We currently have the fairly major (imho) limitation that the entire
     * vector of amples must be read into memory before we can process it.
     */
    #[flame]
    pub fn analyse<T>(self: &mut BpmTools, samples: T) -> f32
    where
        T: Iterator<Item = f32>,
    {
        /* Maintain an energy meter (similar to PPM), and
         * at regular intervals, sample the energy to give a
         * low-resolution overview of the track
         */
        let mut nrg: Vec<f32> = Vec::new(); //with_capacity(samples.len() / self.interval as usize);
        let mut n: u64 = 0;

        let mut v: f32 = 0.0;
        for s in samples {
            let z: f32 = s.abs();
            if z > v {
                v += (z - v) / 8.0;
            } else {
                v -= (v - z) / 512.0;
            }

            n += 1;
            if n == self.interval {
                n = 0;
                nrg.push(v);
            }
        }
        self.scan_for_bpm(&nrg)
    }

    /*
     * Scan a range of BPM values for the one with the
     * minimum autodifference
     */
    #[flame]
    fn scan_for_bpm(self: &mut BpmTools, nrg: &Vec<f32>) -> f32 {
        let slowest = self.bpm_to_interval(self.lower);
        let fastest = self.bpm_to_interval(self.upper);
        let step = (slowest - fastest) / self.steps as f32;

        let mut height = f32::INFINITY;
        let mut trough = f32::NAN;

        // rust won't let us iterate over floats :(
        // write the iteration as a for loop instead
        let mut interval = fastest;
        while interval <= slowest {
            let mut t = 0.0;
            for _ in 0..self.samples {
                t += self.autodifference(&nrg, interval);
            }

            if t < height {
                trough = interval;
                height = t;
            }

            // finish iteration
            interval += step;
        }
        self.interval_to_bpm(trough)
    }

    /*
     * Test an autodifference for the given interval
     */
     #[flame]
    fn autodifference(self: &mut BpmTools, nrg: &Vec<f32>, interval: f32) -> f32 {
        // define some arrays of constants
        const BEATS: [f32; 12] = [
            -32.0, -16.0, -8.0, -4.0, -2.0, -1.0, 1.0, 2.0, 4.0, 8.0, 16.0, 32.0,
        ];
        const NOBEATS: [f32; 4] = [-0.5, -0.25, 0.25, 0.5];

        // until we can generate random numbers, use the mean of the uniform distribution over [0.0, 1.0]
        let side = Uniform::new(0.0, 1.0);
        // const RANDOM_NUMBER: f32 = 0.5;
        let mid: f32 = self.rng.sample(side) * nrg.len() as f32;
        let v: f32 = BpmTools::sample(&nrg, mid);

        let mut diff: f32 = 0.0;
        let mut total: f32 = 0.0;

        for n in 0..BEATS.len() {
            let y: f32 = BpmTools::sample(&nrg, mid + BEATS[n] * interval);
            let w = 1.0 / BEATS[n].abs();

            diff += w * (y - v).abs();
            total += w;
        }

        for n in 0..NOBEATS.len() {
            let y = BpmTools::sample(&nrg, mid + NOBEATS[n] * interval);
            let w = NOBEATS[n].abs();

            diff -= w * (y - v).abs();
            total += w;
        }

        diff / total
    }

    /*
     * Sample from the metered energy
     *
     * No need to interpolate and it makes a tiny amount of difference; we
     * take a random sample of samples, any errors are averaged out.
     */
    fn sample(nrg: &Vec<f32>, offset: f32) -> f32 {
        let n: f32 = offset.floor();
        let i: usize = n as usize; // does this do (in c terms) `i = (u32) n`?

        if n >= 0.0 && n < nrg.len() as f32 {
            nrg[i]
        } else {
            0.0
        }
    }

    /*
     * Beats-per-minute to a sampling interval in energy space
     */
    fn bpm_to_interval(self: &BpmTools, bpm: f32) -> f32 {
        let beats_per_second: f32 = bpm / 60.0;
        let samples_per_beat: f32 = self.rate / beats_per_second;
        samples_per_beat / self.interval as f32
    }

    /*
     * Sampling interval in enery space to beats-per-minute
     */
    fn interval_to_bpm(self: &BpmTools, interval: f32) -> f32 {
        let samples_per_beat: f32 = interval * self.interval as f32;
        let beats_per_second: f32 = self.rate / samples_per_beat;
        beats_per_second * 60.0
    }
}
