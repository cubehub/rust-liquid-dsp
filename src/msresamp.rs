/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Andres Vahter (andres.vahter@gmail.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use ffiliquid;
use num::complex::Complex;
use super::LiquidComplex32;
use std::mem::transmute;

pub struct MsresampCrcf {
     object: ffiliquid::msresamp_crcf,
     resample_rate: f32,
}

impl MsresampCrcf {

    /// Creates multi-stage arbitrary resampler where `resample_rate` is resampling rate expressed as (output/input).
    /// Filter stop-band `attenuation` is in dB.
    pub fn new(resample_rate: f32, attenuation: f32) -> MsresampCrcf {
        let resampler: ffiliquid::msresamp_crcf = unsafe{ffiliquid::msresamp_crcf_create(resample_rate, attenuation)};
        MsresampCrcf{object: resampler,
                     resample_rate: resample_rate
        }
    }

    /// Get filter delay (output samples).
    pub fn get_delay(&self) -> f32 {
        unsafe{ffiliquid::msresamp_crcf_get_delay(self.object)}
    }

    /// Creates Vec<Complex<f32>> that contains resampled original `input` signal.
    pub fn resample(&self, input: &mut [Complex<f32>]) -> Vec<Complex<f32>> {
        let output_len = (2f32 * input.len() as f32 * self.resample_rate) as u32;
        let mut output = vec![Complex::<f32>::new(0.0f32, 0.0f32); output_len as usize];

        let _nx = input.len() as u32;
        let x = unsafe {transmute::<*mut Complex<f32>, *mut LiquidComplex32>(input.as_mut_ptr())};
        let y = unsafe {transmute::<*mut Complex<f32>, *mut LiquidComplex32>(output.as_mut_ptr())};
        let mut output_sample_count = 0;

        // execute multi-stage resampler
        //  _x      :   input sample array  [size: _nx x 1]
        //  _nx     :   input sample array size
        //  _y      :   output sample array [size: variable]
        //  _ny     :   number of samples written to _y
        unsafe {ffiliquid::msresamp_crcf_execute(self.object, x, _nx, y, &mut output_sample_count)};
        output.truncate(output_sample_count as usize);
        output
    }
}

impl Drop for MsresampCrcf {
    fn drop(&mut self) {
        unsafe{ffiliquid::msresamp_crcf_destroy(self.object)};
    }
}
