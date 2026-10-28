

// let h = bound.height.max(BASE_HEIGHT as f32);
// let w = bound.width.max(BASE_WIDTH as f32);

// let target_h = w/ASPECT_RATIO;

// // given a 1:1 box with aspect ratio 1
// // this logic restrains a used for 1:2 box or 2:1 box to be constrained
// // to 1:1, while allowing the UI to grow to 2:2 or 3:3
// // This logic also applies to non square ratios which might be enforced by ASPECT_RATIO
// let final_h = if target_h < h { target_h} else {h};
// let final_w = final_h * ASPECT_RATIO;

// let scale_factor = final_h/BASE_HEIGHT as f32;