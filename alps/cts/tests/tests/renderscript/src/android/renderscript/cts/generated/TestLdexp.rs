/*
 * Copyright (C) 2016 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Don't edit this file!  It is auto-generated by frameworks/rs/api/generate.sh.

#pragma version(1)
#pragma rs java_package_name(android.renderscript.cts)

rs_allocation gAllocInExponent;

float __attribute__((kernel)) testLdexpFloatIntFloat(float inMantissa, unsigned int x) {
    int inExponent = rsGetElementAt_int(gAllocInExponent, x);
    return ldexp(inMantissa, inExponent);
}

float2 __attribute__((kernel)) testLdexpFloat2Int2Float2(float2 inMantissa, unsigned int x) {
    int2 inExponent = rsGetElementAt_int2(gAllocInExponent, x);
    return ldexp(inMantissa, inExponent);
}

float3 __attribute__((kernel)) testLdexpFloat3Int3Float3(float3 inMantissa, unsigned int x) {
    int3 inExponent = rsGetElementAt_int3(gAllocInExponent, x);
    return ldexp(inMantissa, inExponent);
}

float4 __attribute__((kernel)) testLdexpFloat4Int4Float4(float4 inMantissa, unsigned int x) {
    int4 inExponent = rsGetElementAt_int4(gAllocInExponent, x);
    return ldexp(inMantissa, inExponent);
}

float2 __attribute__((kernel)) testLdexpFloat2IntFloat2(float2 inMantissa, unsigned int x) {
    int inExponent = rsGetElementAt_int(gAllocInExponent, x);
    return ldexp(inMantissa, inExponent);
}

float3 __attribute__((kernel)) testLdexpFloat3IntFloat3(float3 inMantissa, unsigned int x) {
    int inExponent = rsGetElementAt_int(gAllocInExponent, x);
    return ldexp(inMantissa, inExponent);
}

float4 __attribute__((kernel)) testLdexpFloat4IntFloat4(float4 inMantissa, unsigned int x) {
    int inExponent = rsGetElementAt_int(gAllocInExponent, x);
    return ldexp(inMantissa, inExponent);
}