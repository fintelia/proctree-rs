use std::{ptr, slice};

pub struct Tree {
    pub vertices: Vec<f32>,
    pub normals: Vec<f32>,
    pub texcoords: Vec<f32>,
    pub twig_vertices: Vec<f32>,
    pub twig_normals: Vec<f32>,
    pub twig_texcoords: Vec<f32>,
    pub faces: Vec<i32>,
    pub twig_faces: Vec<i32>,
}

pub fn generate_tree(random_seed: i32) -> Tree {
    let mut tree = proctree_sys::Tree {
        mProperties: proctree_sys::Properties {
            mSeed: 262,
            mSegments: 6,
            mLevels: 5,
            mVMultiplier: 0.36,
            mTwigScale: 0.39,
            mInitialBranchLength: 0.49,
            mLengthFalloffFactor: 0.85,
            mLengthFalloffPower: 0.99,
            mClumpMax: 0.454,
            mClumpMin: 0.404,
            mBranchFactor: 2.45,
            mDropAmount: -0.1,
            mGrowAmount: 0.235,
            mSweepAmount: 0.01,
            mMaxRadius: 0.139,
            mClimbRate: 0.371,
            mTrunkKink: 0.093,
            mTreeSteps: 5,
            mTaperRate: 0.947,
            mRadiusFalloffRate: 0.73,
            mTwistRate: 3.02,
            mTrunkLength: 2.4,
            mRseed: random_seed,
        },

        mRoot: ptr::null_mut(),
        mVert: ptr::null_mut(),
        mNormal: ptr::null_mut(),
        mUV: ptr::null_mut(),
        mTwigVert: ptr::null_mut(),
        mTwigNormal: ptr::null_mut(),
        mTwigUV: ptr::null_mut(),
        mFace: ptr::null_mut(),
        mTwigFace: ptr::null_mut(),

        mVertCount: 0,
        mTwigVertCount: 0,
        mFaceCount: 0,
        mTwigFaceCount: 0,
    };

    unsafe {
        proctree_sys::Tree_generate(&mut tree as *mut proctree_sys::Tree);
    }

    let mut output = Tree {
        vertices: vec![0.0; tree.mVertCount as usize * 3],
        normals: vec![0.0; tree.mVertCount as usize* 3],
        texcoords: vec![0.0; tree.mVertCount as usize * 2],
        twig_vertices: vec![0.0; tree.mTwigVertCount as usize * 3],
        twig_normals: vec![0.0; tree.mTwigVertCount as usize* 3],
        twig_texcoords: vec![0.0; tree.mTwigVertCount as usize * 2],
        faces: vec![0; tree.mFaceCount as usize * 3],
        twig_faces: vec![0; tree.mTwigFaceCount as usize * 3],
    };

    unsafe {
        output.vertices.copy_from_slice(slice::from_raw_parts(tree.mVert as *const f32, tree.mVertCount as usize * 3));
        output.normals.copy_from_slice(slice::from_raw_parts(tree.mNormal as *const f32, tree.mVertCount as usize* 3));
        output.texcoords.copy_from_slice(slice::from_raw_parts(tree.mUV as *const f32, tree.mVertCount as usize * 2));
        output.twig_vertices.copy_from_slice(slice::from_raw_parts(tree.mTwigVert as *const f32, tree.mTwigVertCount as usize * 3));
        output.twig_normals.copy_from_slice(slice::from_raw_parts(tree.mTwigNormal as *const f32, tree.mTwigVertCount as usize * 3));
        output.twig_texcoords.copy_from_slice(slice::from_raw_parts(tree.mTwigUV as *const f32, tree.mTwigVertCount as usize * 2));
        output.faces.copy_from_slice(slice::from_raw_parts(tree.mFace as *const i32, tree.mFaceCount as usize * 3));
        output.twig_faces.copy_from_slice(slice::from_raw_parts(tree.mTwigFace as *const i32, tree.mTwigFaceCount as usize * 3));
    }

    output
}

#[cfg(test)]
mod tests {
    #[test]
    fn make_tree() {
        let _ = super::generate_tree(12);
    }
}