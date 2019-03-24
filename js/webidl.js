// THIS FILE IS GENERATED FROM tools/generate_webidl.js
import allocator from "./allocator";

function createWebIDLContext() {
  let A = allocator();
  const webidl = {
    allocator: function() {
      return A;
    },

    global_create_f32array: function(start, length) {
      return A.a(
        new Float32Array(
          Uint8Array.from(
            new Uint8Array(this.memory.buffer).subarray(start, start + length)
          ).buffer
        )
      );
    },

    global_create_uint8array: function(start, length) {
      return A.a(
        new Uint8Array(this.memory.buffer).subarray(start, start + length)
      );
    },

    global_is_null: function(o) {
      return A.g(o) == null;
    },

    global_convert_ref_to_string: function(o) {
      return this.ms(o);
    },

    global_debugger: function() {
      debugger;
    },

    global_get_window: function() {
      return A.a(window);
    },

    global_release: function(handle) {
      A.r(handle);
    },

    global_create_event_listener: function() {
      let handle = A.a(e => this.executeCallback(handle, e, A));
      return handle;
    },

    global_get_property: function(handle, name) {
      let o = A.g(handle);
      let p = o[this.s(name)];
      if (typeof p == "string") {
        return this.ms(p);
      } else if (typeof p == "boolean") {
        return p ? 1 : 0;
      } else if (typeof p == "number") {
        return p;
      }
      return A.a(p);
    },

    date_now: function() {
      return Date.now();
    },
    date_now_seconds: function() {
      return Date.now() / 1000;
    },
    date_get_timezone_offset: function() {
      return new Date().getTimezoneOffset();
    },
    math_random: function() {
      return Math.random();
    },
    math_random_n: function(n) {
      return Math.floor(Math.random() * n);
    },
    customelement_attach_shadow: function(instance) {
      let _instance = A.g(instance);
      return A.a(_instance.attachShadow({ mode: "open" }));
    },
    customelement_define: function(componentName) {
      componentName = this.s(componentName);
      let createElement = this.elementCreated;
      window.setTimeout(() => {
        customElements.define(
          componentName,
          class extends HTMLElement {
            constructor() {
              super();
              var e = new CustomEvent("customelementcreated", {
                detail: A.a(this)
              });
              window.dispatchEvent(e);
            }
            connectedCallback() {
              var e = new CustomEvent("connected");
              this.dispatchEvent(e);
            }
            disconnectedCallback() {
              debugger;
              var e = new CustomEvent("disconnected");
              this.dispatchEvent(e);
            }
            adoptedCallback() {
              var e = new CustomEvent("adopted");
              this.dispatchEvent(e);
            }
            attributeChangedCallback(name, oldValue, value) {
              var e = new CustomEvent("attributechanged", {
                detail: { name, oldValue, value }
              });
              this.dispatchEvent(e);
            }
          }
        );
      }, 1);
    },
    customelement_define_with_attributes: function(componentName, attributes) {
      componentName = this.s(componentName);
      attributes = this.s(attributes);
      let createElement = this.elementCreated;
      let observedAttributes = attributes.split(",").map(x => x.trim());
      window.setTimeout(() => {
        customElements.define(
          componentName,
          class extends HTMLElement {
            constructor() {
              super();
              var e = new CustomEvent("customelementcreated", {
                detail: A.a(this)
              });
              window.dispatchEvent(e);
            }
            static get observedAttributes() {
              return observedAttributes;
            }
            connectedCallback() {
              var e = new CustomEvent("connected");
              this.dispatchEvent(e);
            }
            disconnectedCallback() {
              debugger;
              var e = new CustomEvent("disconnected");
              this.dispatchEvent(e);
            }
            adoptedCallback() {
              var e = new CustomEvent("adopted");
              this.dispatchEvent(e);
            }
            attributeChangedCallback(name, oldValue, value) {
              var e = new CustomEvent("attributechanged", {
                detail: { name, oldValue, value }
              });
              this.dispatchEvent(e);
            }
          }
        );
      }, 1);
    },

    WasmWorker_onWorkerLoaded: function(instance, listener) {
      let _instance = A.g(instance);
      let _listener = A.g(listener);
      if (_instance.loaded) {
        _listener(
          new CustomEvent("load", { detail: { id: _instance.workerId } })
        );
      } else {
        _instance.addEventListener("load", _listener);
      }
    },
    WasmWorker_onWorkerMessage: function(instance, listener) {
      let _instance = A.g(instance);
      let _listener = A.g(listener);
      _instance.addEventListener("message", _listener);
    },
    WasmWorker_sendWorkerMessage: function(instance, start, len) {
      let _instance = A.g(instance);
      const data = new Uint8Array(this.memory.buffer);
      _instance.sendMessage(data.subarray(start, start + len));
    },
    WasmWorkerLoadEvent_getWorkerId: function(ev) {
      let e = A.g(ev);
      return e.detail.id;
    },
    WasmWorkerMessageEvent_get_length: function(ev) {
      let e = A.g(ev);
      return e.detail.length;
    },
    WasmWorkerMessageEvent_get_data: function(ev) {
      let e = A.g(ev);
      let start = this.m(e.length);
      const data = new Uint8Array(this.memory.buffer);
      data.set(e.detail, start);
      return start;
    },

    //TODO: get rid of one day when this isn't required by tinygo
    io_get_stdout: function() {
      return 1;
    },

    //TODO: get rid of one day when this isn't required by tinygo
    resource_write: function() {},

    canvas_get_canvas: function(i) {
      let _i = A.g(i);
      return A.a(_i.canvas);
    },

    canvas_set_canvas: function(i, v) {
      let _i = A.g(i);
      _i.canvas = A.g(v);
    },

    canvas_get_moz_current_transform: function(i) {
      let _i = A.g(i);
      return A.a(_i.mozCurrentTransform);
    },

    canvas_set_moz_current_transform: function(i, v) {
      let _i = A.g(i);
      _i.mozCurrentTransform = A.g(v);
    },

    canvas_get_moz_current_transform_inverse: function(i) {
      let _i = A.g(i);
      return A.a(_i.mozCurrentTransformInverse);
    },

    canvas_set_moz_current_transform_inverse: function(i, v) {
      let _i = A.g(i);
      _i.mozCurrentTransformInverse = A.g(v);
    },

    canvas_get_moz_text_style: function(i) {
      let _i = A.g(i);
      return this.ms(_i.mozTextStyle);
    },

    canvas_set_moz_text_style: function(i, v) {
      let _i = A.g(i);
      _i.mozTextStyle = this.s(v);
    },

    canvas_get_moz_image_smoothing_enabled: function(i) {
      let _i = A.g(i);
      return _i.mozImageSmoothingEnabled ? 1 : 0;
    },

    canvas_set_moz_image_smoothing_enabled: function(i, v) {
      let _i = A.g(i);
      _i.mozImageSmoothingEnabled = 1 == v;
    },

    canvas_draw_window: function(i, window, x, y, w, h, bgColor, flags) {
      let _i = A.g(i);

      let _window = A.g(window);
      let _x = x;
      let _y = y;
      let _w = w;
      let _h = h;
      let _bgColor = this.s(bgColor);
      let _flags = flags;
      _i.drawWindow(_window, _x, _y, _w, _h, _bgColor, _flags);
    },

    canvas_demote: function(i) {
      let _i = A.g(i);

      _i.demote();
    },

    canvas_save: function(i) {
      let _i = A.g(i);

      _i.save();
    },

    canvas_restore: function(i) {
      let _i = A.g(i);

      _i.restore();
    },

    canvas_scale: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.scale(_x, _y);
    },

    canvas_rotate: function(i, angle) {
      let _i = A.g(i);

      let _angle = angle;
      _i.rotate(_angle);
    },

    canvas_translate: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.translate(_x, _y);
    },

    canvas_transform: function(i, a, b, c, d, e, f) {
      let _i = A.g(i);

      let _a = a;
      let _b = b;
      let _c = c;
      let _d = d;
      let _e = e;
      let _f = f;
      _i.transform(_a, _b, _c, _d, _e, _f);
    },

    canvas_set_transform: function(i, a, b, c, d, e, f) {
      let _i = A.g(i);

      let _a = a;
      let _b = b;
      let _c = c;
      let _d = d;
      let _e = e;
      let _f = f;
      _i.setTransform(_a, _b, _c, _d, _e, _f);
    },

    canvas_reset_transform: function(i) {
      let _i = A.g(i);

      _i.resetTransform();
    },

    canvas_get_global_alpha: function(i) {
      let _i = A.g(i);
      return _i.globalAlpha;
    },

    canvas_set_global_alpha: function(i, v) {
      let _i = A.g(i);
      _i.globalAlpha = v;
    },

    canvas_get_global_composite_operation: function(i) {
      let _i = A.g(i);
      return this.ms(_i.globalCompositeOperation);
    },

    canvas_set_global_composite_operation: function(i, v) {
      let _i = A.g(i);
      _i.globalCompositeOperation = this.s(v);
    },

    canvas_get_image_smoothing_enabled: function(i) {
      let _i = A.g(i);
      return _i.imageSmoothingEnabled ? 1 : 0;
    },

    canvas_set_image_smoothing_enabled: function(i, v) {
      let _i = A.g(i);
      _i.imageSmoothingEnabled = 1 == v;
    },

    canvas_get_stroke_style: function(i) {
      let _i = A.g(i);
      return A.a(_i.strokeStyle);
    },

    canvas_set_stroke_style: function(i, v) {
      let _i = A.g(i);
      _i.strokeStyle = A.g(v);
    },

    canvas_get_fill_style: function(i) {
      let _i = A.g(i);
      return A.a(_i.fillStyle);
    },

    canvas_set_fill_style: function(i, v) {
      let _i = A.g(i);
      _i.fillStyle = A.g(v);
    },

    canvas_create_linear_gradient: function(i, x0, y0, x1, y1) {
      let _i = A.g(i);

      let _x0 = x0;
      let _y0 = y0;
      let _x1 = x1;
      let _y1 = y1;
      return A.a(_i.createLinearGradient(_x0, _y0, _x1, _y1));
    },

    canvas_create_radial_gradient: function(i, x0, y0, r0, x1, y1, r1) {
      let _i = A.g(i);

      let _x0 = x0;
      let _y0 = y0;
      let _r0 = r0;
      let _x1 = x1;
      let _y1 = y1;
      let _r1 = r1;
      return A.a(_i.createRadialGradient(_x0, _y0, _r0, _x1, _y1, _r1));
    },

    canvas_create_pattern: function(i, image, repetition) {
      let _i = A.g(i);

      let _image = A.g(image);
      let _repetition = this.s(repetition);
      return A.a(_i.createPattern(_image, _repetition));
    },

    canvas_get_shadow_offset_x: function(i) {
      let _i = A.g(i);
      return _i.shadowOffsetX;
    },

    canvas_set_shadow_offset_x: function(i, v) {
      let _i = A.g(i);
      _i.shadowOffsetX = v;
    },

    canvas_get_shadow_offset_y: function(i) {
      let _i = A.g(i);
      return _i.shadowOffsetY;
    },

    canvas_set_shadow_offset_y: function(i, v) {
      let _i = A.g(i);
      _i.shadowOffsetY = v;
    },

    canvas_get_shadow_blur: function(i) {
      let _i = A.g(i);
      return _i.shadowBlur;
    },

    canvas_set_shadow_blur: function(i, v) {
      let _i = A.g(i);
      _i.shadowBlur = v;
    },

    canvas_get_shadow_color: function(i) {
      let _i = A.g(i);
      return this.ms(_i.shadowColor);
    },

    canvas_set_shadow_color: function(i, v) {
      let _i = A.g(i);
      _i.shadowColor = this.s(v);
    },

    canvas_get_filter: function(i) {
      let _i = A.g(i);
      return this.ms(_i.filter);
    },

    canvas_set_filter: function(i, v) {
      let _i = A.g(i);
      _i.filter = this.s(v);
    },

    canvas_clear_rect: function(i, x, y, w, h) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      let _w = w;
      let _h = h;
      _i.clearRect(_x, _y, _w, _h);
    },

    canvas_fill_rect: function(i, x, y, w, h) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      let _w = w;
      let _h = h;
      _i.fillRect(_x, _y, _w, _h);
    },

    canvas_stroke_rect: function(i, x, y, w, h) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      let _w = w;
      let _h = h;
      _i.strokeRect(_x, _y, _w, _h);
    },

    canvas_begin_path: function(i) {
      let _i = A.g(i);

      _i.beginPath();
    },

    canvas_fill: function(i, winding) {
      let _i = A.g(i);

      let _winding = A.g(winding);
      _i.fill(_winding);
    },

    canvas_fill_1: function(i, path, winding) {
      let _i = A.g(i);

      let _path = A.g(path);
      let _winding = A.g(winding);
      _i.fill(_path, _winding);
    },

    canvas_stroke: function(i) {
      let _i = A.g(i);

      _i.stroke();
    },

    canvas_stroke_1: function(i, path) {
      let _i = A.g(i);

      let _path = A.g(path);
      _i.stroke(_path);
    },

    canvas_clip: function(i, winding) {
      let _i = A.g(i);

      let _winding = A.g(winding);
      _i.clip(_winding);
    },

    canvas_clip_1: function(i, path, winding) {
      let _i = A.g(i);

      let _path = A.g(path);
      let _winding = A.g(winding);
      _i.clip(_path, _winding);
    },

    canvas_is_point_in_path: function(i, x, y, winding) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      let _winding = A.g(winding);
      return A.a(_i.isPointInPath(_x, _y, _winding)) ? 1 : 0;
    },

    canvas_is_point_in_path_1: function(i, path, x, y, winding) {
      let _i = A.g(i);

      let _path = A.g(path);
      let _x = x;
      let _y = y;
      let _winding = A.g(winding);
      return A.a(_i.isPointInPath(_path, _x, _y, _winding)) ? 1 : 0;
    },

    canvas_is_point_in_stroke: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      return A.a(_i.isPointInStroke(_x, _y)) ? 1 : 0;
    },

    canvas_is_point_in_stroke_1: function(i, path, x, y) {
      let _i = A.g(i);

      let _path = A.g(path);
      let _x = x;
      let _y = y;
      return A.a(_i.isPointInStroke(_path, _x, _y)) ? 1 : 0;
    },

    canvas_draw_focus_if_needed: function(i, element) {
      let _i = A.g(i);

      let _element = A.g(element);
      _i.drawFocusIfNeeded(_element);
    },

    canvas_fill_text: function(i, text, x, y, maxWidth) {
      let _i = A.g(i);

      let _text = this.s(text);
      let _x = x;
      let _y = y;
      let _maxWidth = maxWidth;
      _i.fillText(_text, _x, _y, _maxWidth);
    },

    canvas_stroke_text: function(i, text, x, y, maxWidth) {
      let _i = A.g(i);

      let _text = this.s(text);
      let _x = x;
      let _y = y;
      let _maxWidth = maxWidth;
      _i.strokeText(_text, _x, _y, _maxWidth);
    },

    canvas_measure_text: function(i, text) {
      let _i = A.g(i);

      let _text = this.s(text);
      return A.a(_i.measureText(_text));
    },

    canvas_draw_image: function(i, image, dx, dy) {
      let _i = A.g(i);

      let _image = A.g(image);
      let _dx = dx;
      let _dy = dy;
      _i.drawImage(_image, _dx, _dy);
    },

    canvas_draw_image_1: function(i, image, dx, dy, dw, dh) {
      let _i = A.g(i);

      let _image = A.g(image);
      let _dx = dx;
      let _dy = dy;
      let _dw = dw;
      let _dh = dh;
      _i.drawImage(_image, _dx, _dy, _dw, _dh);
    },

    canvas_draw_image_2: function(i, image, sx, sy, sw, sh, dx, dy, dw, dh) {
      let _i = A.g(i);

      let _image = A.g(image);
      let _sx = sx;
      let _sy = sy;
      let _sw = sw;
      let _sh = sh;
      let _dx = dx;
      let _dy = dy;
      let _dw = dw;
      let _dh = dh;
      _i.drawImage(_image, _sx, _sy, _sw, _sh, _dx, _dy, _dw, _dh);
    },

    canvas_create_image_data: function(i, sw, sh) {
      let _i = A.g(i);

      let _sw = sw;
      let _sh = sh;
      return A.a(_i.createImageData(_sw, _sh));
    },

    canvas_create_image_data_1: function(i, imagedata) {
      let _i = A.g(i);

      let _imagedata = A.g(imagedata);
      return A.a(_i.createImageData(_imagedata));
    },

    canvas_get_image_data: function(i, sx, sy, sw, sh) {
      let _i = A.g(i);

      let _sx = sx;
      let _sy = sy;
      let _sw = sw;
      let _sh = sh;
      return A.a(_i.getImageData(_sx, _sy, _sw, _sh));
    },

    canvas_put_image_data: function(i, imagedata, dx, dy) {
      let _i = A.g(i);

      let _imagedata = A.g(imagedata);
      let _dx = dx;
      let _dy = dy;
      _i.putImageData(_imagedata, _dx, _dy);
    },

    canvas_put_image_data_1: function(
      i,
      imagedata,
      dx,
      dy,
      dirtyX,
      dirtyY,
      dirtyWidth,
      dirtyHeight
    ) {
      let _i = A.g(i);

      let _imagedata = A.g(imagedata);
      let _dx = dx;
      let _dy = dy;
      let _dirtyX = dirtyX;
      let _dirtyY = dirtyY;
      let _dirtyWidth = dirtyWidth;
      let _dirtyHeight = dirtyHeight;
      _i.putImageData(
        _imagedata,
        _dx,
        _dy,
        _dirtyX,
        _dirtyY,
        _dirtyWidth,
        _dirtyHeight
      );
    },

    canvas_get_line_width: function(i) {
      let _i = A.g(i);
      return _i.lineWidth;
    },

    canvas_set_line_width: function(i, v) {
      let _i = A.g(i);
      _i.lineWidth = v;
    },

    canvas_get_line_cap: function(i) {
      let _i = A.g(i);
      return this.ms(_i.lineCap);
    },

    canvas_set_line_cap: function(i, v) {
      let _i = A.g(i);
      _i.lineCap = this.s(v);
    },

    canvas_get_line_join: function(i) {
      let _i = A.g(i);
      return this.ms(_i.lineJoin);
    },

    canvas_set_line_join: function(i, v) {
      let _i = A.g(i);
      _i.lineJoin = this.s(v);
    },

    canvas_get_miter_limit: function(i) {
      let _i = A.g(i);
      return _i.miterLimit;
    },

    canvas_set_miter_limit: function(i, v) {
      let _i = A.g(i);
      _i.miterLimit = v;
    },

    canvas_set_line_dash: function(i, segments) {
      let _i = A.g(i);

      let _segments = A.g(segments);
      _i.setLineDash(_segments);
    },

    canvas_get_line_dash: function(i) {
      let _i = A.g(i);

      return A.a(_i.getLineDash());
    },

    canvas_get_line_dash_offset: function(i) {
      let _i = A.g(i);
      return _i.lineDashOffset;
    },

    canvas_set_line_dash_offset: function(i, v) {
      let _i = A.g(i);
      _i.lineDashOffset = v;
    },

    canvas_get_font: function(i) {
      let _i = A.g(i);
      return this.ms(_i.font);
    },

    canvas_set_font: function(i, v) {
      let _i = A.g(i);
      _i.font = this.s(v);
    },

    canvas_get_text_align: function(i) {
      let _i = A.g(i);
      return this.ms(_i.textAlign);
    },

    canvas_set_text_align: function(i, v) {
      let _i = A.g(i);
      _i.textAlign = this.s(v);
    },

    canvas_get_text_baseline: function(i) {
      let _i = A.g(i);
      return this.ms(_i.textBaseline);
    },

    canvas_set_text_baseline: function(i, v) {
      let _i = A.g(i);
      _i.textBaseline = this.s(v);
    },

    canvas_close_path: function(i) {
      let _i = A.g(i);

      _i.closePath();
    },

    canvas_move_to: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.moveTo(_x, _y);
    },

    canvas_line_to: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.lineTo(_x, _y);
    },

    canvas_quadratic_curve_to: function(i, cpx, cpy, x, y) {
      let _i = A.g(i);

      let _cpx = cpx;
      let _cpy = cpy;
      let _x = x;
      let _y = y;
      _i.quadraticCurveTo(_cpx, _cpy, _x, _y);
    },

    canvas_bezier_curve_to: function(i, cp1x, cp1y, cp2x, cp2y, x, y) {
      let _i = A.g(i);

      let _cp1x = cp1x;
      let _cp1y = cp1y;
      let _cp2x = cp2x;
      let _cp2y = cp2y;
      let _x = x;
      let _y = y;
      _i.bezierCurveTo(_cp1x, _cp1y, _cp2x, _cp2y, _x, _y);
    },

    canvas_arc_to: function(i, x1, y1, x2, y2, radius) {
      let _i = A.g(i);

      let _x1 = x1;
      let _y1 = y1;
      let _x2 = x2;
      let _y2 = y2;
      let _radius = radius;
      _i.arcTo(_x1, _y1, _x2, _y2, _radius);
    },

    canvas_rect: function(i, x, y, w, h) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      let _w = w;
      let _h = h;
      _i.rect(_x, _y, _w, _h);
    },

    canvas_arc: function(i, x, y, radius, startAngle, endAngle, anticlockwise) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      let _radius = radius;
      let _startAngle = startAngle;
      let _endAngle = endAngle;
      let _anticlockwise = 0 != anticlockwise;
      _i.arc(_x, _y, _radius, _startAngle, _endAngle, _anticlockwise);
    },

    canvas_ellipse: function(
      i,
      x,
      y,
      radiusX,
      radiusY,
      rotation,
      startAngle,
      endAngle,
      anticlockwise
    ) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      let _radiusX = radiusX;
      let _radiusY = radiusY;
      let _rotation = rotation;
      let _startAngle = startAngle;
      let _endAngle = endAngle;
      let _anticlockwise = 0 != anticlockwise;
      _i.ellipse(
        _x,
        _y,
        _radiusX,
        _radiusY,
        _rotation,
        _startAngle,
        _endAngle,
        _anticlockwise
      );
    },

    canvas_add_hit_region: function(i, options) {
      let _i = A.g(i);

      let _options = A.g(options);
      _i.addHitRegion(_options);
    },

    canvas_remove_hit_region: function(i, id) {
      let _i = A.g(i);

      let _id = this.s(id);
      _i.removeHitRegion(_id);
    },

    canvas_clear_hit_regions: function(i) {
      let _i = A.g(i);

      _i.clearHitRegions();
    },

    canvasgradient_add_color_stop: function(i, offset, color) {
      let _i = A.g(i);

      let _offset = offset;
      let _color = this.s(color);
      _i.addColorStop(_offset, _color);
    },

    canvaspattern_set_transform: function(i, matrix) {
      let _i = A.g(i);

      let _matrix = A.g(matrix);
      _i.setTransform(_matrix);
    },

    textmetrics_get_width: function(i) {
      let _i = A.g(i);
      return _i.width;
    },

    textmetrics_set_width: function(i, v) {
      let _i = A.g(i);
      _i.width = v;
    },

    path2d_add_path: function(i, path, transformation) {
      let _i = A.g(i);

      let _path = A.g(path);
      let _transformation = A.g(transformation);
      _i.addPath(_path, _transformation);
    },

    path2d_close_path: function(i) {
      let _i = A.g(i);

      _i.closePath();
    },

    path2d_move_to: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.moveTo(_x, _y);
    },

    path2d_line_to: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.lineTo(_x, _y);
    },

    path2d_quadratic_curve_to: function(i, cpx, cpy, x, y) {
      let _i = A.g(i);

      let _cpx = cpx;
      let _cpy = cpy;
      let _x = x;
      let _y = y;
      _i.quadraticCurveTo(_cpx, _cpy, _x, _y);
    },

    path2d_bezier_curve_to: function(i, cp1x, cp1y, cp2x, cp2y, x, y) {
      let _i = A.g(i);

      let _cp1x = cp1x;
      let _cp1y = cp1y;
      let _cp2x = cp2x;
      let _cp2y = cp2y;
      let _x = x;
      let _y = y;
      _i.bezierCurveTo(_cp1x, _cp1y, _cp2x, _cp2y, _x, _y);
    },

    path2d_arc_to: function(i, x1, y1, x2, y2, radius) {
      let _i = A.g(i);

      let _x1 = x1;
      let _y1 = y1;
      let _x2 = x2;
      let _y2 = y2;
      let _radius = radius;
      _i.arcTo(_x1, _y1, _x2, _y2, _radius);
    },

    path2d_rect: function(i, x, y, w, h) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      let _w = w;
      let _h = h;
      _i.rect(_x, _y, _w, _h);
    },

    path2d_arc: function(i, x, y, radius, startAngle, endAngle, anticlockwise) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      let _radius = radius;
      let _startAngle = startAngle;
      let _endAngle = endAngle;
      let _anticlockwise = 0 != anticlockwise;
      _i.arc(_x, _y, _radius, _startAngle, _endAngle, _anticlockwise);
    },

    path2d_ellipse: function(
      i,
      x,
      y,
      radiusX,
      radiusY,
      rotation,
      startAngle,
      endAngle,
      anticlockwise
    ) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      let _radiusX = radiusX;
      let _radiusY = radiusY;
      let _rotation = rotation;
      let _startAngle = startAngle;
      let _endAngle = endAngle;
      let _anticlockwise = 0 != anticlockwise;
      _i.ellipse(
        _x,
        _y,
        _radiusX,
        _radiusY,
        _rotation,
        _startAngle,
        _endAngle,
        _anticlockwise
      );
    },

    console_assert: function(condition, message) {
      let _condition = 0 != condition;
      let _message = this.s(message);
      console.assert(_condition, _message);
    },

    console_clear: function() {
      console.clear();
    },

    console_count: function(label) {
      let _label = this.s(label);
      console.count(_label);
    },

    console_count_reset: function(label) {
      let _label = this.s(label);
      console.countReset(_label);
    },

    console_debug: function(message) {
      let _message = this.s(message);
      console.debug(_message);
    },

    console_error: function(message) {
      let _message = this.s(message);
      console.error(_message);
    },

    console_info: function(message) {
      let _message = this.s(message);
      console.info(_message);
    },

    console_log: function(message) {
      let _message = this.s(message);
      console.log(_message);
    },

    console_table: function(message) {
      let _message = this.s(message);
      console.table(_message);
    },

    console_trace: function(message) {
      let _message = this.s(message);
      console.trace(_message);
    },

    console_warn: function(message) {
      let _message = this.s(message);
      console.warn(_message);
    },

    console_dir: function(message) {
      let _message = this.s(message);
      console.dir(_message);
    },

    console_dirxml: function(message) {
      let _message = this.s(message);
      console.dirxml(_message);
    },

    console_group: function(message) {
      let _message = this.s(message);
      console.group(_message);
    },

    console_group_collapsed: function(message) {
      let _message = this.s(message);
      console.groupCollapsed(_message);
    },

    console_group_end: function() {
      console.groupEnd();
    },

    console_time: function(label) {
      let _label = this.s(label);
      console.time(_label);
    },

    console_time_log: function(label, message) {
      let _label = this.s(label);
      let _message = this.s(message);
      console.timeLog(_label, _message);
    },

    console_time_end: function(label) {
      let _label = this.s(label);
      console.timeEnd(_label);
    },

    console_exception: function(message) {
      let _message = this.s(message);
      console.exception(_message);
    },

    console_time_stamp: function(message) {
      let _message = this.s(message);
      console.timeStamp(_message);
    },

    console_profile: function(message) {
      let _message = this.s(message);
      console.profile(_message);
    },

    console_profile_end: function(message) {
      let _message = this.s(message);
      console.profileEnd(_message);
    },

    consoleinstance_assert: function(i, condition, message) {
      let _i = A.g(i);

      let _condition = 0 != condition;
      let _message = this.s(message);
      _i.assert(_condition, _message);
    },

    consoleinstance_clear: function(i) {
      let _i = A.g(i);

      _i.clear();
    },

    consoleinstance_count: function(i, label) {
      let _i = A.g(i);

      let _label = this.s(label);
      _i.count(_label);
    },

    consoleinstance_count_reset: function(i, label) {
      let _i = A.g(i);

      let _label = this.s(label);
      _i.countReset(_label);
    },

    consoleinstance_debug: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.debug(_message);
    },

    consoleinstance_error: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.error(_message);
    },

    consoleinstance_info: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.info(_message);
    },

    consoleinstance_log: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.log(_message);
    },

    consoleinstance_table: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.table(_message);
    },

    consoleinstance_trace: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.trace(_message);
    },

    consoleinstance_warn: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.warn(_message);
    },

    consoleinstance_dir: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.dir(_message);
    },

    consoleinstance_dirxml: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.dirxml(_message);
    },

    consoleinstance_group: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.group(_message);
    },

    consoleinstance_group_collapsed: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.groupCollapsed(_message);
    },

    consoleinstance_group_end: function(i) {
      let _i = A.g(i);

      _i.groupEnd();
    },

    consoleinstance_time: function(i, label) {
      let _i = A.g(i);

      let _label = this.s(label);
      _i.time(_label);
    },

    consoleinstance_time_log: function(i, label, message) {
      let _i = A.g(i);

      let _label = this.s(label);
      let _message = this.s(message);
      _i.timeLog(_label, _message);
    },

    consoleinstance_time_end: function(i, label) {
      let _i = A.g(i);

      let _label = this.s(label);
      _i.timeEnd(_label);
    },

    consoleinstance_exception: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.exception(_message);
    },

    consoleinstance_time_stamp: function(i, data) {
      let _i = A.g(i);

      let _data = A.g(data);
      _i.timeStamp(_data);
    },

    consoleinstance_profile: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.profile(_message);
    },

    consoleinstance_profile_end: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.profileEnd(_message);
    },

    consoleinstance_report_for_service_worker_scope: function(
      i,
      scope,
      message,
      filename,
      lineNumber,
      columnNumber,
      level
    ) {
      let _i = A.g(i);

      let _scope = this.s(scope);
      let _message = this.s(message);
      let _filename = this.s(filename);
      let _lineNumber = lineNumber;
      let _columnNumber = columnNumber;
      let _level = A.g(level);
      _i.reportForServiceWorkerScope(
        _scope,
        _message,
        _filename,
        _lineNumber,
        _columnNumber,
        _level
      );
    },

    document_get_implementation: function(i) {
      let _i = A.g(i);
      return A.a(_i.implementation);
    },

    document_set_implementation: function(i, v) {
      let _i = A.g(i);
      _i.implementation = A.g(v);
    },

    document_get_url: function(i) {
      let _i = A.g(i);
      return this.ms(_i.URL);
    },

    document_set_url: function(i, v) {
      let _i = A.g(i);
      _i.URL = this.s(v);
    },

    document_get_document_uri: function(i) {
      let _i = A.g(i);
      return this.ms(_i.documentURI);
    },

    document_set_document_uri: function(i, v) {
      let _i = A.g(i);
      _i.documentURI = this.s(v);
    },

    document_get_compat_mode: function(i) {
      let _i = A.g(i);
      return this.ms(_i.compatMode);
    },

    document_set_compat_mode: function(i, v) {
      let _i = A.g(i);
      _i.compatMode = this.s(v);
    },

    document_get_character_set: function(i) {
      let _i = A.g(i);
      return this.ms(_i.characterSet);
    },

    document_set_character_set: function(i, v) {
      let _i = A.g(i);
      _i.characterSet = this.s(v);
    },

    document_get_charset: function(i) {
      let _i = A.g(i);
      return this.ms(_i.charset);
    },

    document_set_charset: function(i, v) {
      let _i = A.g(i);
      _i.charset = this.s(v);
    },

    document_get_input_encoding: function(i) {
      let _i = A.g(i);
      return this.ms(_i.inputEncoding);
    },

    document_set_input_encoding: function(i, v) {
      let _i = A.g(i);
      _i.inputEncoding = this.s(v);
    },

    document_get_content_type: function(i) {
      let _i = A.g(i);
      return this.ms(_i.contentType);
    },

    document_set_content_type: function(i, v) {
      let _i = A.g(i);
      _i.contentType = this.s(v);
    },

    document_get_doctype: function(i) {
      let _i = A.g(i);
      return A.a(_i.doctype);
    },

    document_set_doctype: function(i, v) {
      let _i = A.g(i);
      _i.doctype = A.g(v);
    },

    document_get_document_element: function(i) {
      let _i = A.g(i);
      return A.a(_i.documentElement);
    },

    document_set_document_element: function(i, v) {
      let _i = A.g(i);
      _i.documentElement = A.g(v);
    },

    document_get_elements_by_tag_name: function(i, localName) {
      let _i = A.g(i);

      let _localName = this.s(localName);
      return A.a(_i.getElementsByTagName(_localName));
    },

    document_get_elements_by_tag_name_n_s: function(i, namespace, localName) {
      let _i = A.g(i);

      let _namespace = this.s(namespace);
      let _localName = this.s(localName);
      return A.a(_i.getElementsByTagNameNS(_namespace, _localName));
    },

    document_get_elements_by_class_name: function(i, classNames) {
      let _i = A.g(i);

      let _classNames = this.s(classNames);
      return A.a(_i.getElementsByClassName(_classNames));
    },

    document_get_element_by_id: function(i, elementId) {
      let _i = A.g(i);

      let _elementId = this.s(elementId);
      return A.a(_i.getElementById(_elementId));
    },

    document_create_element: function(i, localName, options) {
      let _i = A.g(i);

      let _localName = this.s(localName);
      let _options = A.g(options);
      return A.a(_i.createElement(_localName, _options));
    },

    document_create_element_n_s: function(
      i,
      namespace,
      qualifiedName,
      options
    ) {
      let _i = A.g(i);

      let _namespace = this.s(namespace);
      let _qualifiedName = this.s(qualifiedName);
      let _options = A.g(options);
      return A.a(_i.createElementNS(_namespace, _qualifiedName, _options));
    },

    document_create_document_fragment: function(i) {
      let _i = A.g(i);

      return A.a(_i.createDocumentFragment());
    },

    document_create_text_node: function(i, data) {
      let _i = A.g(i);

      let _data = this.s(data);
      return A.a(_i.createTextNode(_data));
    },

    document_create_comment: function(i, data) {
      let _i = A.g(i);

      let _data = this.s(data);
      return A.a(_i.createComment(_data));
    },

    document_create_processing_instruction: function(i, target, data) {
      let _i = A.g(i);

      let _target = this.s(target);
      let _data = this.s(data);
      return A.a(_i.createProcessingInstruction(_target, _data));
    },

    document_import_node: function(i, node, deep) {
      let _i = A.g(i);

      let _node = A.g(node);
      let _deep = 0 != deep;
      return A.a(_i.importNode(_node, _deep));
    },

    document_adopt_node: function(i, node) {
      let _i = A.g(i);

      let _node = A.g(node);
      return A.a(_i.adoptNode(_node));
    },

    document_create_event: function(i, name) {
      let _i = A.g(i);

      let _name = this.s(name);
      return A.a(_i.createEvent(_name));
    },

    document_create_range: function(i) {
      let _i = A.g(i);

      return A.a(_i.createRange());
    },

    document_create_node_iterator: function(i, root, whatToShow, filter) {
      let _i = A.g(i);

      let _root = A.g(root);
      let _whatToShow = whatToShow;
      let _filter = A.g(filter);
      return A.a(_i.createNodeIterator(_root, _whatToShow, _filter));
    },

    document_create_tree_walker: function(i, root, whatToShow, filter) {
      let _i = A.g(i);

      let _root = A.g(root);
      let _whatToShow = whatToShow;
      let _filter = A.g(filter);
      return A.a(_i.createTreeWalker(_root, _whatToShow, _filter));
    },

    document_create_c_d_a_t_a_section: function(i, data) {
      let _i = A.g(i);

      let _data = this.s(data);
      return A.a(_i.createCDATASection(_data));
    },

    document_create_attribute: function(i, name) {
      let _i = A.g(i);

      let _name = this.s(name);
      return A.a(_i.createAttribute(_name));
    },

    document_create_attribute_n_s: function(i, namespace, name) {
      let _i = A.g(i);

      let _namespace = this.s(namespace);
      let _name = this.s(name);
      return A.a(_i.createAttributeNS(_namespace, _name));
    },

    document_get_location: function(i) {
      let _i = A.g(i);
      return A.a(_i.location);
    },

    document_set_location: function(i, v) {
      let _i = A.g(i);
      _i.location = A.g(v);
    },

    document_get_referrer: function(i) {
      let _i = A.g(i);
      return this.ms(_i.referrer);
    },

    document_set_referrer: function(i, v) {
      let _i = A.g(i);
      _i.referrer = this.s(v);
    },

    document_get_last_modified: function(i) {
      let _i = A.g(i);
      return this.ms(_i.lastModified);
    },

    document_set_last_modified: function(i, v) {
      let _i = A.g(i);
      _i.lastModified = this.s(v);
    },

    document_get_ready_state: function(i) {
      let _i = A.g(i);
      return this.ms(_i.readyState);
    },

    document_set_ready_state: function(i, v) {
      let _i = A.g(i);
      _i.readyState = this.s(v);
    },

    document_get_title: function(i) {
      let _i = A.g(i);
      return this.ms(_i.title);
    },

    document_set_title: function(i, v) {
      let _i = A.g(i);
      _i.title = this.s(v);
    },

    document_get_dir: function(i) {
      let _i = A.g(i);
      return this.ms(_i.dir);
    },

    document_set_dir: function(i, v) {
      let _i = A.g(i);
      _i.dir = this.s(v);
    },

    document_get_body: function(i) {
      let _i = A.g(i);
      return A.a(_i.body);
    },

    document_set_body: function(i, v) {
      let _i = A.g(i);
      _i.body = A.g(v);
    },

    document_get_head: function(i) {
      let _i = A.g(i);
      return A.a(_i.head);
    },

    document_set_head: function(i, v) {
      let _i = A.g(i);
      _i.head = A.g(v);
    },

    document_get_images: function(i) {
      let _i = A.g(i);
      return A.a(_i.images);
    },

    document_set_images: function(i, v) {
      let _i = A.g(i);
      _i.images = A.g(v);
    },

    document_get_embeds: function(i) {
      let _i = A.g(i);
      return A.a(_i.embeds);
    },

    document_set_embeds: function(i, v) {
      let _i = A.g(i);
      _i.embeds = A.g(v);
    },

    document_get_plugins: function(i) {
      let _i = A.g(i);
      return A.a(_i.plugins);
    },

    document_set_plugins: function(i, v) {
      let _i = A.g(i);
      _i.plugins = A.g(v);
    },

    document_get_links: function(i) {
      let _i = A.g(i);
      return A.a(_i.links);
    },

    document_set_links: function(i, v) {
      let _i = A.g(i);
      _i.links = A.g(v);
    },

    document_get_forms: function(i) {
      let _i = A.g(i);
      return A.a(_i.forms);
    },

    document_set_forms: function(i, v) {
      let _i = A.g(i);
      _i.forms = A.g(v);
    },

    document_get_scripts: function(i) {
      let _i = A.g(i);
      return A.a(_i.scripts);
    },

    document_set_scripts: function(i, v) {
      let _i = A.g(i);
      _i.scripts = A.g(v);
    },

    document_get_elements_by_name: function(i, elementName) {
      let _i = A.g(i);

      let _elementName = this.s(elementName);
      return A.a(_i.getElementsByName(_elementName));
    },

    document_get_default_view: function(i) {
      let _i = A.g(i);
      return A.a(_i.defaultView);
    },

    document_set_default_view: function(i, v) {
      let _i = A.g(i);
      _i.defaultView = A.g(v);
    },

    document_has_focus: function(i) {
      let _i = A.g(i);

      return A.a(_i.hasFocus()) ? 1 : 0;
    },

    document_get_onreadystatechange: function(i) {
      let _i = A.g(i);
      return A.a(_i.onreadystatechange);
    },

    document_set_onreadystatechange: function(i, v) {
      let _i = A.g(i);
      _i.onreadystatechange = A.g(v);
    },

    document_get_onbeforescriptexecute: function(i) {
      let _i = A.g(i);
      return A.a(_i.onbeforescriptexecute);
    },

    document_set_onbeforescriptexecute: function(i, v) {
      let _i = A.g(i);
      _i.onbeforescriptexecute = A.g(v);
    },

    document_get_onafterscriptexecute: function(i) {
      let _i = A.g(i);
      return A.a(_i.onafterscriptexecute);
    },

    document_set_onafterscriptexecute: function(i, v) {
      let _i = A.g(i);
      _i.onafterscriptexecute = A.g(v);
    },

    document_get_onselectionchange: function(i) {
      let _i = A.g(i);
      return A.a(_i.onselectionchange);
    },

    document_set_onselectionchange: function(i, v) {
      let _i = A.g(i);
      _i.onselectionchange = A.g(v);
    },

    document_get_current_script: function(i) {
      let _i = A.g(i);
      return A.a(_i.currentScript);
    },

    document_set_current_script: function(i, v) {
      let _i = A.g(i);
      _i.currentScript = A.g(v);
    },

    document_release_capture: function(i) {
      let _i = A.g(i);

      _i.releaseCapture();
    },

    document_get_document_uri_object: function(i) {
      let _i = A.g(i);
      return A.a(_i.documentURIObject);
    },

    document_set_document_uri_object: function(i, v) {
      let _i = A.g(i);
      _i.documentURIObject = A.g(v);
    },

    document_get_referrer_policy: function(i) {
      let _i = A.g(i);
      return _i.referrerPolicy;
    },

    document_set_referrer_policy: function(i, v) {
      let _i = A.g(i);
      _i.referrerPolicy = v;
    },

    document_get_anchors: function(i) {
      let _i = A.g(i);
      return A.a(_i.anchors);
    },

    document_set_anchors: function(i, v) {
      let _i = A.g(i);
      _i.anchors = A.g(v);
    },

    document_get_applets: function(i) {
      let _i = A.g(i);
      return A.a(_i.applets);
    },

    document_set_applets: function(i, v) {
      let _i = A.g(i);
      _i.applets = A.g(v);
    },

    document_get_fullscreen: function(i) {
      let _i = A.g(i);
      return _i.fullscreen ? 1 : 0;
    },

    document_set_fullscreen: function(i, v) {
      let _i = A.g(i);
      _i.fullscreen = 1 == v;
    },

    document_get_fullscreen_enabled: function(i) {
      let _i = A.g(i);
      return _i.fullscreenEnabled ? 1 : 0;
    },

    document_set_fullscreen_enabled: function(i, v) {
      let _i = A.g(i);
      _i.fullscreenEnabled = 1 == v;
    },

    document_exit_fullscreen: function(i) {
      let _i = A.g(i);

      _i.exitFullscreen();
    },

    document_get_onfullscreenchange: function(i) {
      let _i = A.g(i);
      return A.a(_i.onfullscreenchange);
    },

    document_set_onfullscreenchange: function(i, v) {
      let _i = A.g(i);
      _i.onfullscreenchange = A.g(v);
    },

    document_get_onfullscreenerror: function(i) {
      let _i = A.g(i);
      return A.a(_i.onfullscreenerror);
    },

    document_set_onfullscreenerror: function(i, v) {
      let _i = A.g(i);
      _i.onfullscreenerror = A.g(v);
    },

    document_exit_pointer_lock: function(i) {
      let _i = A.g(i);

      _i.exitPointerLock();
    },

    document_get_onpointerlockchange: function(i) {
      let _i = A.g(i);
      return A.a(_i.onpointerlockchange);
    },

    document_set_onpointerlockchange: function(i, v) {
      let _i = A.g(i);
      _i.onpointerlockchange = A.g(v);
    },

    document_get_onpointerlockerror: function(i) {
      let _i = A.g(i);
      return A.a(_i.onpointerlockerror);
    },

    document_set_onpointerlockerror: function(i, v) {
      let _i = A.g(i);
      _i.onpointerlockerror = A.g(v);
    },

    document_get_hidden: function(i) {
      let _i = A.g(i);
      return _i.hidden ? 1 : 0;
    },

    document_set_hidden: function(i, v) {
      let _i = A.g(i);
      _i.hidden = 1 == v;
    },

    document_get_visibility_state: function(i) {
      let _i = A.g(i);
      return A.a(_i.visibilityState);
    },

    document_set_visibility_state: function(i, v) {
      let _i = A.g(i);
      _i.visibilityState = A.g(v);
    },

    document_get_onvisibilitychange: function(i) {
      let _i = A.g(i);
      return A.a(_i.onvisibilitychange);
    },

    document_set_onvisibilitychange: function(i, v) {
      let _i = A.g(i);
      _i.onvisibilitychange = A.g(v);
    },

    document_get_selected_style_sheet_set: function(i) {
      let _i = A.g(i);
      return this.ms(_i.selectedStyleSheetSet);
    },

    document_set_selected_style_sheet_set: function(i, v) {
      let _i = A.g(i);
      _i.selectedStyleSheetSet = this.s(v);
    },

    document_get_last_style_sheet_set: function(i) {
      let _i = A.g(i);
      return this.ms(_i.lastStyleSheetSet);
    },

    document_set_last_style_sheet_set: function(i, v) {
      let _i = A.g(i);
      _i.lastStyleSheetSet = this.s(v);
    },

    document_get_preferred_style_sheet_set: function(i) {
      let _i = A.g(i);
      return this.ms(_i.preferredStyleSheetSet);
    },

    document_set_preferred_style_sheet_set: function(i, v) {
      let _i = A.g(i);
      _i.preferredStyleSheetSet = this.s(v);
    },

    document_get_style_sheet_sets: function(i) {
      let _i = A.g(i);
      return A.a(_i.styleSheetSets);
    },

    document_set_style_sheet_sets: function(i, v) {
      let _i = A.g(i);
      _i.styleSheetSets = A.g(v);
    },

    document_enable_style_sheets_for_set: function(i, name) {
      let _i = A.g(i);

      let _name = this.s(name);
      _i.enableStyleSheetsForSet(_name);
    },

    document_caret_position_from_point: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      return A.a(_i.caretPositionFromPoint(_x, _y));
    },

    document_get_scrolling_element: function(i) {
      let _i = A.g(i);
      return A.a(_i.scrollingElement);
    },

    document_set_scrolling_element: function(i, v) {
      let _i = A.g(i);
      _i.scrollingElement = A.g(v);
    },

    document_query_selector: function(i, selectors) {
      let _i = A.g(i);

      let _selectors = this.s(selectors);
      return A.a(_i.querySelector(_selectors));
    },

    document_query_selector_all: function(i, selectors) {
      let _i = A.g(i);

      let _selectors = this.s(selectors);
      return A.a(_i.querySelectorAll(_selectors));
    },

    document_get_timeline: function(i) {
      let _i = A.g(i);
      return A.a(_i.timeline);
    },

    document_set_timeline: function(i, v) {
      let _i = A.g(i);
      _i.timeline = A.g(v);
    },

    document_get_animations: function(i) {
      let _i = A.g(i);

      return A.a(_i.getAnimations());
    },

    document_get_root_element: function(i) {
      let _i = A.g(i);
      return A.a(_i.rootElement);
    },

    document_set_root_element: function(i, v) {
      let _i = A.g(i);
      _i.rootElement = A.g(v);
    },

    document_get_is_srcdoc_document: function(i) {
      let _i = A.g(i);
      return _i.isSrcdocDocument ? 1 : 0;
    },

    document_set_is_srcdoc_document: function(i, v) {
      let _i = A.g(i);
      _i.isSrcdocDocument = 1 == v;
    },

    document_get_sandbox_flags_as_string: function(i) {
      let _i = A.g(i);
      return this.ms(_i.sandboxFlagsAsString);
    },

    document_set_sandbox_flags_as_string: function(i, v) {
      let _i = A.g(i);
      _i.sandboxFlagsAsString = this.s(v);
    },

    document_insert_anonymous_content: function(i, aElement) {
      let _i = A.g(i);

      let _aElement = A.g(aElement);
      return A.a(_i.insertAnonymousContent(_aElement));
    },

    document_remove_anonymous_content: function(i, aContent) {
      let _i = A.g(i);

      let _aContent = A.g(aContent);
      _i.removeAnonymousContent(_aContent);
    },

    document_get_selection: function(i) {
      let _i = A.g(i);

      return A.a(_i.getSelection());
    },

    document_get_user_has_interacted: function(i) {
      let _i = A.g(i);
      return _i.userHasInteracted ? 1 : 0;
    },

    document_set_user_has_interacted: function(i, v) {
      let _i = A.g(i);
      _i.userHasInteracted = 1 == v;
    },

    document_notify_user_gesture_activation: function(i) {
      let _i = A.g(i);

      _i.notifyUserGestureActivation();
    },

    document_get_document_flash_classification: function(i) {
      let _i = A.g(i);
      return A.a(_i.documentFlashClassification);
    },

    document_set_document_flash_classification: function(i, v) {
      let _i = A.g(i);
      _i.documentFlashClassification = A.g(v);
    },

    element_get_namespace_uri: function(i) {
      let _i = A.g(i);
      return this.ms(_i.namespaceURI);
    },

    element_set_namespace_uri: function(i, v) {
      let _i = A.g(i);
      _i.namespaceURI = this.s(v);
    },

    element_get_prefix: function(i) {
      let _i = A.g(i);
      return this.ms(_i.prefix);
    },

    element_set_prefix: function(i, v) {
      let _i = A.g(i);
      _i.prefix = this.s(v);
    },

    element_get_local_name: function(i) {
      let _i = A.g(i);
      return this.ms(_i.localName);
    },

    element_set_local_name: function(i, v) {
      let _i = A.g(i);
      _i.localName = this.s(v);
    },

    element_get_tag_name: function(i) {
      let _i = A.g(i);
      return this.ms(_i.tagName);
    },

    element_set_tag_name: function(i, v) {
      let _i = A.g(i);
      _i.tagName = this.s(v);
    },

    element_get_id: function(i) {
      let _i = A.g(i);
      return this.ms(_i.id);
    },

    element_set_id: function(i, v) {
      let _i = A.g(i);
      _i.id = this.s(v);
    },

    element_get_class_name: function(i) {
      let _i = A.g(i);
      return this.ms(_i.className);
    },

    element_set_class_name: function(i, v) {
      let _i = A.g(i);
      _i.className = this.s(v);
    },

    element_get_class_list: function(i) {
      let _i = A.g(i);
      return A.a(_i.classList);
    },

    element_set_class_list: function(i, v) {
      let _i = A.g(i);
      _i.classList = A.g(v);
    },

    element_get_attributes: function(i) {
      let _i = A.g(i);
      return A.a(_i.attributes);
    },

    element_set_attributes: function(i, v) {
      let _i = A.g(i);
      _i.attributes = A.g(v);
    },

    element_get_attribute_names: function(i) {
      let _i = A.g(i);

      return A.a(_i.getAttributeNames());
    },

    element_get_attribute: function(i, name) {
      let _i = A.g(i);

      let _name = this.s(name);
      return this.ms(_i.getAttribute(_name));
    },

    element_get_attribute_n_s: function(i, namespace, localName) {
      let _i = A.g(i);

      let _namespace = this.s(namespace);
      let _localName = this.s(localName);
      return this.ms(_i.getAttributeNS(_namespace, _localName));
    },

    element_toggle_attribute: function(i, name, force) {
      let _i = A.g(i);

      let _name = this.s(name);
      let _force = 0 != force;
      return A.a(_i.toggleAttribute(_name, _force)) ? 1 : 0;
    },

    element_set_attribute: function(i, name, value) {
      let _i = A.g(i);

      let _name = this.s(name);
      let _value = this.s(value);
      _i.setAttribute(_name, _value);
    },

    element_set_attribute_n_s: function(i, namespace, name, value) {
      let _i = A.g(i);

      let _namespace = this.s(namespace);
      let _name = this.s(name);
      let _value = this.s(value);
      _i.setAttributeNS(_namespace, _name, _value);
    },

    element_remove_attribute: function(i, name) {
      let _i = A.g(i);

      let _name = this.s(name);
      _i.removeAttribute(_name);
    },

    element_remove_attribute_n_s: function(i, namespace, localName) {
      let _i = A.g(i);

      let _namespace = this.s(namespace);
      let _localName = this.s(localName);
      _i.removeAttributeNS(_namespace, _localName);
    },

    element_has_attribute: function(i, name) {
      let _i = A.g(i);

      let _name = this.s(name);
      return A.a(_i.hasAttribute(_name)) ? 1 : 0;
    },

    element_has_attribute_n_s: function(i, namespace, localName) {
      let _i = A.g(i);

      let _namespace = this.s(namespace);
      let _localName = this.s(localName);
      return A.a(_i.hasAttributeNS(_namespace, _localName)) ? 1 : 0;
    },

    element_has_attributes: function(i) {
      let _i = A.g(i);

      return A.a(_i.hasAttributes()) ? 1 : 0;
    },

    element_closest: function(i, selector) {
      let _i = A.g(i);

      let _selector = this.s(selector);
      return A.a(_i.closest(_selector));
    },

    element_matches: function(i, selector) {
      let _i = A.g(i);

      let _selector = this.s(selector);
      return A.a(_i.matches(_selector)) ? 1 : 0;
    },

    element_webkit_matches_selector: function(i, selector) {
      let _i = A.g(i);

      let _selector = this.s(selector);
      return A.a(_i.webkitMatchesSelector(_selector)) ? 1 : 0;
    },

    element_get_elements_with_grid: function(i) {
      let _i = A.g(i);

      return A.a(_i.getElementsWithGrid());
    },

    element_insert_adjacent_element: function(i, location, element) {
      let _i = A.g(i);

      let _location = this.s(location);
      let _element = A.g(element);
      return A.a(_i.insertAdjacentElement(_location, _element));
    },

    element_insert_adjacent_text: function(i, location, data) {
      let _i = A.g(i);

      let _location = this.s(location);
      let _data = this.s(data);
      _i.insertAdjacentText(_location, _data);
    },

    element_get_font_size_inflation: function(i) {
      let _i = A.g(i);
      return _i.fontSizeInflation;
    },

    element_set_font_size_inflation: function(i, v) {
      let _i = A.g(i);
      _i.fontSizeInflation = v;
    },

    element_set_pointer_capture: function(i, pointerId) {
      let _i = A.g(i);

      let _pointerId = pointerId;
      _i.setPointerCapture(_pointerId);
    },

    element_release_pointer_capture: function(i, pointerId) {
      let _i = A.g(i);

      let _pointerId = pointerId;
      _i.releasePointerCapture(_pointerId);
    },

    element_has_pointer_capture: function(i, pointerId) {
      let _i = A.g(i);

      let _pointerId = pointerId;
      return A.a(_i.hasPointerCapture(_pointerId)) ? 1 : 0;
    },

    element_set_capture: function(i, retargetToElement) {
      let _i = A.g(i);

      let _retargetToElement = 0 != retargetToElement;
      _i.setCapture(_retargetToElement);
    },

    element_release_capture: function(i) {
      let _i = A.g(i);

      _i.releaseCapture();
    },

    element_set_capture_always: function(i, retargetToElement) {
      let _i = A.g(i);

      let _retargetToElement = 0 != retargetToElement;
      _i.setCaptureAlways(_retargetToElement);
    },

    element_get_attribute_node: function(i, name) {
      let _i = A.g(i);

      let _name = this.s(name);
      return A.a(_i.getAttributeNode(_name));
    },

    element_set_attribute_node: function(i, newAttr) {
      let _i = A.g(i);

      let _newAttr = A.g(newAttr);
      return A.a(_i.setAttributeNode(_newAttr));
    },

    element_remove_attribute_node: function(i, oldAttr) {
      let _i = A.g(i);

      let _oldAttr = A.g(oldAttr);
      return A.a(_i.removeAttributeNode(_oldAttr));
    },

    element_get_attribute_node_n_s: function(i, namespaceURI, localName) {
      let _i = A.g(i);

      let _namespaceURI = this.s(namespaceURI);
      let _localName = this.s(localName);
      return A.a(_i.getAttributeNodeNS(_namespaceURI, _localName));
    },

    element_set_attribute_node_n_s: function(i, newAttr) {
      let _i = A.g(i);

      let _newAttr = A.g(newAttr);
      return A.a(_i.setAttributeNodeNS(_newAttr));
    },

    element_scroll_by_no_flush: function(i, dx, dy) {
      let _i = A.g(i);

      let _dx = dx;
      let _dy = dy;
      return A.a(_i.scrollByNoFlush(_dx, _dy)) ? 1 : 0;
    },

    element_get_as_flex_container: function(i) {
      let _i = A.g(i);

      return A.a(_i.getAsFlexContainer());
    },

    element_get_grid_fragments: function(i) {
      let _i = A.g(i);

      return A.a(_i.getGridFragments());
    },

    element_get_transform_to_ancestor: function(i, ancestor) {
      let _i = A.g(i);

      let _ancestor = A.g(ancestor);
      return A.a(_i.getTransformToAncestor(_ancestor));
    },

    element_get_transform_to_parent: function(i) {
      let _i = A.g(i);

      return A.a(_i.getTransformToParent());
    },

    element_get_transform_to_viewport: function(i) {
      let _i = A.g(i);

      return A.a(_i.getTransformToViewport());
    },

    element_get_client_rects: function(i) {
      let _i = A.g(i);

      return A.a(_i.getClientRects());
    },

    element_get_bounding_client_rect: function(i) {
      let _i = A.g(i);

      return A.a(_i.getBoundingClientRect());
    },

    element_scroll_into_view: function(i, arg) {
      let _i = A.g(i);

      let _arg = A.g(arg);
      _i.scrollIntoView(_arg);
    },

    element_get_scroll_top: function(i) {
      let _i = A.g(i);
      return _i.scrollTop;
    },

    element_set_scroll_top: function(i, v) {
      let _i = A.g(i);
      _i.scrollTop = v;
    },

    element_get_scroll_left: function(i) {
      let _i = A.g(i);
      return _i.scrollLeft;
    },

    element_set_scroll_left: function(i, v) {
      let _i = A.g(i);
      _i.scrollLeft = v;
    },

    element_get_scroll_width: function(i) {
      let _i = A.g(i);
      return _i.scrollWidth;
    },

    element_set_scroll_width: function(i, v) {
      let _i = A.g(i);
      _i.scrollWidth = v;
    },

    element_get_scroll_height: function(i) {
      let _i = A.g(i);
      return _i.scrollHeight;
    },

    element_set_scroll_height: function(i, v) {
      let _i = A.g(i);
      _i.scrollHeight = v;
    },

    element_scroll: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.scroll(_x, _y);
    },

    element_scroll_to: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.scrollTo(_x, _y);
    },

    element_scroll_by: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.scrollBy(_x, _y);
    },

    element_get_client_top: function(i) {
      let _i = A.g(i);
      return _i.clientTop;
    },

    element_set_client_top: function(i, v) {
      let _i = A.g(i);
      _i.clientTop = v;
    },

    element_get_client_left: function(i) {
      let _i = A.g(i);
      return _i.clientLeft;
    },

    element_set_client_left: function(i, v) {
      let _i = A.g(i);
      _i.clientLeft = v;
    },

    element_get_client_width: function(i) {
      let _i = A.g(i);
      return _i.clientWidth;
    },

    element_set_client_width: function(i, v) {
      let _i = A.g(i);
      _i.clientWidth = v;
    },

    element_get_client_height: function(i) {
      let _i = A.g(i);
      return _i.clientHeight;
    },

    element_set_client_height: function(i, v) {
      let _i = A.g(i);
      _i.clientHeight = v;
    },

    element_get_inner_html: function(i) {
      let _i = A.g(i);
      return this.ms(_i.innerHTML);
    },

    element_set_inner_html: function(i, v) {
      let _i = A.g(i);
      _i.innerHTML = this.s(v);
    },

    element_get_outer_html: function(i) {
      let _i = A.g(i);
      return this.ms(_i.outerHTML);
    },

    element_set_outer_html: function(i, v) {
      let _i = A.g(i);
      _i.outerHTML = this.s(v);
    },

    element_insert_adjacent_html: function(i, position, text) {
      let _i = A.g(i);

      let _position = this.s(position);
      let _text = this.s(text);
      _i.insertAdjacentHTML(_position, _text);
    },

    element_query_selector: function(i, selectors) {
      let _i = A.g(i);

      let _selectors = this.s(selectors);
      return A.a(_i.querySelector(_selectors));
    },

    element_query_selector_all: function(i, selectors) {
      let _i = A.g(i);

      let _selectors = this.s(selectors);
      return A.a(_i.querySelectorAll(_selectors));
    },

    element_get_shadow_root: function(i) {
      let _i = A.g(i);
      return A.a(_i.shadowRoot);
    },

    element_set_shadow_root: function(i, v) {
      let _i = A.g(i);
      _i.shadowRoot = A.g(v);
    },

    element_get_open_or_closed_shadow_root: function(i) {
      let _i = A.g(i);
      return A.a(_i.openOrClosedShadowRoot);
    },

    element_set_open_or_closed_shadow_root: function(i, v) {
      let _i = A.g(i);
      _i.openOrClosedShadowRoot = A.g(v);
    },

    element_get_assigned_slot: function(i) {
      let _i = A.g(i);
      return A.a(_i.assignedSlot);
    },

    element_set_assigned_slot: function(i, v) {
      let _i = A.g(i);
      _i.assignedSlot = A.g(v);
    },

    element_get_slot: function(i) {
      let _i = A.g(i);
      return this.ms(_i.slot);
    },

    element_set_slot: function(i, v) {
      let _i = A.g(i);
      _i.slot = this.s(v);
    },

    element_request_fullscreen: function(i) {
      let _i = A.g(i);

      _i.requestFullscreen();
    },

    element_request_pointer_lock: function(i) {
      let _i = A.g(i);

      _i.requestPointerLock();
    },

    element_get_children: function(i) {
      let _i = A.g(i);
      return A.a(_i.children);
    },

    element_set_children: function(i, v) {
      let _i = A.g(i);
      _i.children = A.g(v);
    },

    element_get_first_element_child: function(i) {
      let _i = A.g(i);
      return A.a(_i.firstElementChild);
    },

    element_set_first_element_child: function(i, v) {
      let _i = A.g(i);
      _i.firstElementChild = A.g(v);
    },

    element_get_last_element_child: function(i) {
      let _i = A.g(i);
      return A.a(_i.lastElementChild);
    },

    element_set_last_element_child: function(i, v) {
      let _i = A.g(i);
      _i.lastElementChild = A.g(v);
    },

    element_get_child_element_count: function(i) {
      let _i = A.g(i);
      return _i.childElementCount;
    },

    element_set_child_element_count: function(i, v) {
      let _i = A.g(i);
      _i.childElementCount = v;
    },

    element_prepend: function(i, nodes) {
      let _i = A.g(i);

      let _nodes = A.g(nodes);
      _i.prepend(_nodes);
    },

    element_append: function(i, nodes) {
      let _i = A.g(i);

      let _nodes = A.g(nodes);
      _i.append(_nodes);
    },

    element_before: function(i, nodes) {
      let _i = A.g(i);

      let _nodes = A.g(nodes);
      _i.before(_nodes);
    },

    element_after: function(i, nodes) {
      let _i = A.g(i);

      let _nodes = A.g(nodes);
      _i.after(_nodes);
    },

    element_replace_with: function(i, nodes) {
      let _i = A.g(i);

      let _nodes = A.g(nodes);
      _i.replaceWith(_nodes);
    },

    element_remove: function(i) {
      let _i = A.g(i);

      _i.remove();
    },

    element_get_previous_element_sibling: function(i) {
      let _i = A.g(i);
      return A.a(_i.previousElementSibling);
    },

    element_set_previous_element_sibling: function(i, v) {
      let _i = A.g(i);
      _i.previousElementSibling = A.g(v);
    },

    element_get_next_element_sibling: function(i) {
      let _i = A.g(i);
      return A.a(_i.nextElementSibling);
    },

    element_set_next_element_sibling: function(i, v) {
      let _i = A.g(i);
      _i.nextElementSibling = A.g(v);
    },

    eventtarget_add_event_listener: function(i, eventType, listener) {
      let _i = A.g(i);

      let _eventType = this.s(eventType);
      let _listener = A.g(listener);
      _i.addEventListener(_eventType, _listener);
    },

    eventtarget_remove_event_listener: function(i, eventType, listener) {
      let _i = A.g(i);

      let _eventType = this.s(eventType);
      let _listener = A.g(listener);
      _i.removeEventListener(_eventType, _listener);
    },

    eventtarget_dispatch_event: function(i, event) {
      let _i = A.g(i);

      let _event = A.g(event);
      return A.a(_i.dispatchEvent(_event)) ? 1 : 0;
    },

    htmlcanvas_get_width: function(i) {
      let _i = A.g(i);
      return _i.width;
    },

    htmlcanvas_set_width: function(i, v) {
      let _i = A.g(i);
      _i.width = v;
    },

    htmlcanvas_get_height: function(i) {
      let _i = A.g(i);
      return _i.height;
    },

    htmlcanvas_set_height: function(i, v) {
      let _i = A.g(i);
      _i.height = v;
    },

    htmlcanvas_get_context: function(i, contextId) {
      let _i = A.g(i);

      let _contextId = this.s(contextId);
      return A.a(_i.getContext(_contextId));
    },

    htmlcanvas_to_data_url: function(i, dataType, encoderOptions) {
      let _i = A.g(i);

      let _dataType = this.s(dataType);
      let _encoderOptions = A.g(encoderOptions);
      return this.ms(_i.toDataURL(_dataType, _encoderOptions));
    },

    htmlcanvas_to_blob: function(i, callback, blobType, encoderOptions) {
      let _i = A.g(i);

      let _callback = A.g(callback);
      let _blobType = this.s(blobType);
      let _encoderOptions = A.g(encoderOptions);
      _i.toBlob(_callback, _blobType, _encoderOptions);
    },

    htmlcanvas_transfer_control_to_offscreen: function(i) {
      let _i = A.g(i);

      return A.a(_i.transferControlToOffscreen());
    },

    htmlinput_get_accept: function(i) {
      let _i = A.g(i);
      return this.ms(_i.accept);
    },

    htmlinput_set_accept: function(i, v) {
      let _i = A.g(i);
      _i.accept = this.s(v);
    },

    htmlinput_get_alt: function(i) {
      let _i = A.g(i);
      return this.ms(_i.alt);
    },

    htmlinput_set_alt: function(i, v) {
      let _i = A.g(i);
      _i.alt = this.s(v);
    },

    htmlinput_get_autocomplete: function(i) {
      let _i = A.g(i);
      return this.ms(_i.autocomplete);
    },

    htmlinput_set_autocomplete: function(i, v) {
      let _i = A.g(i);
      _i.autocomplete = this.s(v);
    },

    htmlinput_get_autofocus: function(i) {
      let _i = A.g(i);
      return _i.autofocus ? 1 : 0;
    },

    htmlinput_set_autofocus: function(i, v) {
      let _i = A.g(i);
      _i.autofocus = 1 == v;
    },

    htmlinput_get_default_checked: function(i) {
      let _i = A.g(i);
      return _i.defaultChecked ? 1 : 0;
    },

    htmlinput_set_default_checked: function(i, v) {
      let _i = A.g(i);
      _i.defaultChecked = 1 == v;
    },

    htmlinput_get_checked: function(i) {
      let _i = A.g(i);
      return _i.checked ? 1 : 0;
    },

    htmlinput_set_checked: function(i, v) {
      let _i = A.g(i);
      _i.checked = 1 == v;
    },

    htmlinput_get_disabled: function(i) {
      let _i = A.g(i);
      return _i.disabled ? 1 : 0;
    },

    htmlinput_set_disabled: function(i, v) {
      let _i = A.g(i);
      _i.disabled = 1 == v;
    },

    htmlinput_get_form: function(i) {
      let _i = A.g(i);
      return A.a(_i.form);
    },

    htmlinput_set_form: function(i, v) {
      let _i = A.g(i);
      _i.form = A.g(v);
    },

    htmlinput_get_files: function(i) {
      let _i = A.g(i);
      return A.a(_i.files);
    },

    htmlinput_set_files: function(i, v) {
      let _i = A.g(i);
      _i.files = A.g(v);
    },

    htmlinput_get_form_action: function(i) {
      let _i = A.g(i);
      return this.ms(_i.formAction);
    },

    htmlinput_set_form_action: function(i, v) {
      let _i = A.g(i);
      _i.formAction = this.s(v);
    },

    htmlinput_get_form_enctype: function(i) {
      let _i = A.g(i);
      return this.ms(_i.formEnctype);
    },

    htmlinput_set_form_enctype: function(i, v) {
      let _i = A.g(i);
      _i.formEnctype = this.s(v);
    },

    htmlinput_get_form_method: function(i) {
      let _i = A.g(i);
      return this.ms(_i.formMethod);
    },

    htmlinput_set_form_method: function(i, v) {
      let _i = A.g(i);
      _i.formMethod = this.s(v);
    },

    htmlinput_get_form_no_validate: function(i) {
      let _i = A.g(i);
      return _i.formNoValidate ? 1 : 0;
    },

    htmlinput_set_form_no_validate: function(i, v) {
      let _i = A.g(i);
      _i.formNoValidate = 1 == v;
    },

    htmlinput_get_form_target: function(i) {
      let _i = A.g(i);
      return this.ms(_i.formTarget);
    },

    htmlinput_set_form_target: function(i, v) {
      let _i = A.g(i);
      _i.formTarget = this.s(v);
    },

    htmlinput_get_height: function(i) {
      let _i = A.g(i);
      return _i.height;
    },

    htmlinput_set_height: function(i, v) {
      let _i = A.g(i);
      _i.height = v;
    },

    htmlinput_get_indeterminate: function(i) {
      let _i = A.g(i);
      return _i.indeterminate ? 1 : 0;
    },

    htmlinput_set_indeterminate: function(i, v) {
      let _i = A.g(i);
      _i.indeterminate = 1 == v;
    },

    htmlinput_get_input_mode: function(i) {
      let _i = A.g(i);
      return this.ms(_i.inputMode);
    },

    htmlinput_set_input_mode: function(i, v) {
      let _i = A.g(i);
      _i.inputMode = this.s(v);
    },

    htmlinput_get_list: function(i) {
      let _i = A.g(i);
      return A.a(_i.list);
    },

    htmlinput_set_list: function(i, v) {
      let _i = A.g(i);
      _i.list = A.g(v);
    },

    htmlinput_get_max: function(i) {
      let _i = A.g(i);
      return this.ms(_i.max);
    },

    htmlinput_set_max: function(i, v) {
      let _i = A.g(i);
      _i.max = this.s(v);
    },

    htmlinput_get_max_length: function(i) {
      let _i = A.g(i);
      return _i.maxLength;
    },

    htmlinput_set_max_length: function(i, v) {
      let _i = A.g(i);
      _i.maxLength = v;
    },

    htmlinput_get_min: function(i) {
      let _i = A.g(i);
      return this.ms(_i.min);
    },

    htmlinput_set_min: function(i, v) {
      let _i = A.g(i);
      _i.min = this.s(v);
    },

    htmlinput_get_min_length: function(i) {
      let _i = A.g(i);
      return _i.minLength;
    },

    htmlinput_set_min_length: function(i, v) {
      let _i = A.g(i);
      _i.minLength = v;
    },

    htmlinput_get_multiple: function(i) {
      let _i = A.g(i);
      return _i.multiple ? 1 : 0;
    },

    htmlinput_set_multiple: function(i, v) {
      let _i = A.g(i);
      _i.multiple = 1 == v;
    },

    htmlinput_get_name: function(i) {
      let _i = A.g(i);
      return this.ms(_i.name);
    },

    htmlinput_set_name: function(i, v) {
      let _i = A.g(i);
      _i.name = this.s(v);
    },

    htmlinput_get_pattern: function(i) {
      let _i = A.g(i);
      return this.ms(_i.pattern);
    },

    htmlinput_set_pattern: function(i, v) {
      let _i = A.g(i);
      _i.pattern = this.s(v);
    },

    htmlinput_get_placeholder: function(i) {
      let _i = A.g(i);
      return this.ms(_i.placeholder);
    },

    htmlinput_set_placeholder: function(i, v) {
      let _i = A.g(i);
      _i.placeholder = this.s(v);
    },

    htmlinput_get_read_only: function(i) {
      let _i = A.g(i);
      return _i.readOnly ? 1 : 0;
    },

    htmlinput_set_read_only: function(i, v) {
      let _i = A.g(i);
      _i.readOnly = 1 == v;
    },

    htmlinput_get_required: function(i) {
      let _i = A.g(i);
      return _i.required ? 1 : 0;
    },

    htmlinput_set_required: function(i, v) {
      let _i = A.g(i);
      _i.required = 1 == v;
    },

    htmlinput_get_size: function(i) {
      let _i = A.g(i);
      return _i.size;
    },

    htmlinput_set_size: function(i, v) {
      let _i = A.g(i);
      _i.size = v;
    },

    htmlinput_get_src: function(i) {
      let _i = A.g(i);
      return this.ms(_i.src);
    },

    htmlinput_set_src: function(i, v) {
      let _i = A.g(i);
      _i.src = this.s(v);
    },

    htmlinput_get_step: function(i) {
      let _i = A.g(i);
      return this.ms(_i.step);
    },

    htmlinput_set_step: function(i, v) {
      let _i = A.g(i);
      _i.step = this.s(v);
    },

    htmlinput_get_type: function(i) {
      let _i = A.g(i);
      return this.ms(_i.type);
    },

    htmlinput_set_type: function(i, v) {
      let _i = A.g(i);
      _i.type = this.s(v);
    },

    htmlinput_get_default_value: function(i) {
      let _i = A.g(i);
      return this.ms(_i.defaultValue);
    },

    htmlinput_set_default_value: function(i, v) {
      let _i = A.g(i);
      _i.defaultValue = this.s(v);
    },

    htmlinput_get_value: function(i) {
      let _i = A.g(i);
      return this.ms(_i.value);
    },

    htmlinput_set_value: function(i, v) {
      let _i = A.g(i);
      _i.value = this.s(v);
    },

    htmlinput_get_value_as_date: function(i) {
      let _i = A.g(i);
      return A.a(_i.valueAsDate);
    },

    htmlinput_set_value_as_date: function(i, v) {
      let _i = A.g(i);
      _i.valueAsDate = A.g(v);
    },

    htmlinput_get_value_as_number: function(i) {
      let _i = A.g(i);
      return _i.valueAsNumber;
    },

    htmlinput_set_value_as_number: function(i, v) {
      let _i = A.g(i);
      _i.valueAsNumber = v;
    },

    htmlinput_get_width: function(i) {
      let _i = A.g(i);
      return _i.width;
    },

    htmlinput_set_width: function(i, v) {
      let _i = A.g(i);
      _i.width = v;
    },

    htmlinput_get_will_validate: function(i) {
      let _i = A.g(i);
      return _i.willValidate ? 1 : 0;
    },

    htmlinput_set_will_validate: function(i, v) {
      let _i = A.g(i);
      _i.willValidate = 1 == v;
    },

    htmlinput_get_validity: function(i) {
      let _i = A.g(i);
      return A.a(_i.validity);
    },

    htmlinput_set_validity: function(i, v) {
      let _i = A.g(i);
      _i.validity = A.g(v);
    },

    htmlinput_get_validation_message: function(i) {
      let _i = A.g(i);
      return this.ms(_i.validationMessage);
    },

    htmlinput_set_validation_message: function(i, v) {
      let _i = A.g(i);
      _i.validationMessage = this.s(v);
    },

    htmlinput_check_validity: function(i) {
      let _i = A.g(i);

      return A.a(_i.checkValidity()) ? 1 : 0;
    },

    htmlinput_report_validity: function(i) {
      let _i = A.g(i);

      return A.a(_i.reportValidity()) ? 1 : 0;
    },

    htmlinput_set_custom_validity: function(i, error) {
      let _i = A.g(i);

      let _error = this.s(error);
      _i.setCustomValidity(_error);
    },

    htmlinput_get_labels: function(i) {
      let _i = A.g(i);
      return A.a(_i.labels);
    },

    htmlinput_set_labels: function(i, v) {
      let _i = A.g(i);
      _i.labels = A.g(v);
    },

    htmlinput_select: function(i) {
      let _i = A.g(i);

      _i.select();
    },

    htmlinput_get_selection_direction: function(i) {
      let _i = A.g(i);
      return this.ms(_i.selectionDirection);
    },

    htmlinput_set_selection_direction: function(i, v) {
      let _i = A.g(i);
      _i.selectionDirection = this.s(v);
    },

    htmlinput_set_range_text: function(
      i,
      replacement,
      start,
      end,
      selectionMode
    ) {
      let _i = A.g(i);

      let _replacement = this.s(replacement);
      let _start = start;
      let _end = end;
      let _selectionMode = A.g(selectionMode);
      _i.setRangeText(_replacement, _start, _end, _selectionMode);
    },

    htmlinput_set_selection_range: function(i, start, end, direction) {
      let _i = A.g(i);

      let _start = start;
      let _end = end;
      let _direction = this.s(direction);
      _i.setSelectionRange(_start, _end, _direction);
    },

    htmlinput_get_align: function(i) {
      let _i = A.g(i);
      return this.ms(_i.align);
    },

    htmlinput_set_align: function(i, v) {
      let _i = A.g(i);
      _i.align = this.s(v);
    },

    htmlinput_get_use_map: function(i) {
      let _i = A.g(i);
      return this.ms(_i.useMap);
    },

    htmlinput_set_use_map: function(i, v) {
      let _i = A.g(i);
      _i.useMap = this.s(v);
    },

    htmlinput_get_date_time_input_box_value: function(i) {
      let _i = A.g(i);

      return A.a(_i.getDateTimeInputBoxValue());
    },

    htmlinput_update_date_time_input_box: function(i, value) {
      let _i = A.g(i);

      let _value = A.g(value);
      _i.updateDateTimeInputBox(_value);
    },

    htmlinput_set_date_time_picker_state: function(i, open) {
      let _i = A.g(i);

      let _open = 0 != open;
      _i.setDateTimePickerState(_open);
    },

    htmlinput_get_minimum: function(i) {
      let _i = A.g(i);

      return A.a(_i.getMinimum());
    },

    htmlinput_get_maximum: function(i) {
      let _i = A.g(i);

      return A.a(_i.getMaximum());
    },

    htmlinput_get_preview_value: function(i) {
      let _i = A.g(i);
      return this.ms(_i.previewValue);
    },

    htmlinput_set_preview_value: function(i, v) {
      let _i = A.g(i);
      _i.previewValue = this.s(v);
    },

    keyboardevent_get_char_code: function(i) {
      let _i = A.g(i);
      return _i.charCode;
    },

    keyboardevent_set_char_code: function(i, v) {
      let _i = A.g(i);
      _i.charCode = v;
    },

    keyboardevent_get_key_code: function(i) {
      let _i = A.g(i);
      return _i.keyCode;
    },

    keyboardevent_set_key_code: function(i, v) {
      let _i = A.g(i);
      _i.keyCode = v;
    },

    keyboardevent_get_alt_key: function(i) {
      let _i = A.g(i);
      return _i.altKey ? 1 : 0;
    },

    keyboardevent_set_alt_key: function(i, v) {
      let _i = A.g(i);
      _i.altKey = 1 == v;
    },

    keyboardevent_get_ctrl_key: function(i) {
      let _i = A.g(i);
      return _i.ctrlKey ? 1 : 0;
    },

    keyboardevent_set_ctrl_key: function(i, v) {
      let _i = A.g(i);
      _i.ctrlKey = 1 == v;
    },

    keyboardevent_get_shift_key: function(i) {
      let _i = A.g(i);
      return _i.shiftKey ? 1 : 0;
    },

    keyboardevent_set_shift_key: function(i, v) {
      let _i = A.g(i);
      _i.shiftKey = 1 == v;
    },

    keyboardevent_get_meta_key: function(i) {
      let _i = A.g(i);
      return _i.metaKey ? 1 : 0;
    },

    keyboardevent_set_meta_key: function(i, v) {
      let _i = A.g(i);
      _i.metaKey = 1 == v;
    },

    keyboardevent_get_modifier_state: function(i, key) {
      let _i = A.g(i);

      let _key = this.s(key);
      return A.a(_i.getModifierState(_key)) ? 1 : 0;
    },

    keyboardevent_get_location: function(i) {
      let _i = A.g(i);
      return _i.location;
    },

    keyboardevent_set_location: function(i, v) {
      let _i = A.g(i);
      _i.location = v;
    },

    keyboardevent_get_repeat: function(i) {
      let _i = A.g(i);
      return _i.repeat ? 1 : 0;
    },

    keyboardevent_set_repeat: function(i, v) {
      let _i = A.g(i);
      _i.repeat = 1 == v;
    },

    keyboardevent_get_is_composing: function(i) {
      let _i = A.g(i);
      return _i.isComposing ? 1 : 0;
    },

    keyboardevent_set_is_composing: function(i, v) {
      let _i = A.g(i);
      _i.isComposing = 1 == v;
    },

    keyboardevent_get_key: function(i) {
      let _i = A.g(i);
      return this.ms(_i.key);
    },

    keyboardevent_set_key: function(i, v) {
      let _i = A.g(i);
      _i.key = this.s(v);
    },

    keyboardevent_get_code: function(i) {
      let _i = A.g(i);
      return this.ms(_i.code);
    },

    keyboardevent_set_code: function(i, v) {
      let _i = A.g(i);
      _i.code = this.s(v);
    },

    keyboardevent_init_keyboard_event: function(
      i,
      typeArg,
      bubblesArg,
      cancelableArg,
      viewArg,
      keyArg,
      locationArg,
      ctrlKey,
      altKey,
      shiftKey,
      metaKey
    ) {
      let _i = A.g(i);

      let _typeArg = this.s(typeArg);
      let _bubblesArg = 0 != bubblesArg;
      let _cancelableArg = 0 != cancelableArg;
      let _viewArg = A.g(viewArg);
      let _keyArg = this.s(keyArg);
      let _locationArg = locationArg;
      let _ctrlKey = 0 != ctrlKey;
      let _altKey = 0 != altKey;
      let _shiftKey = 0 != shiftKey;
      let _metaKey = 0 != metaKey;
      _i.initKeyboardEvent(
        _typeArg,
        _bubblesArg,
        _cancelableArg,
        _viewArg,
        _keyArg,
        _locationArg,
        _ctrlKey,
        _altKey,
        _shiftKey,
        _metaKey
      );
    },

    keyboardevent_get_init_dict: function(i) {
      let _i = A.g(i);
      return A.a(_i.initDict);
    },

    keyboardevent_set_init_dict: function(i, v) {
      let _i = A.g(i);
      _i.initDict = A.g(v);
    },

    mouseevent_get_screen_x: function(i) {
      let _i = A.g(i);
      return _i.screenX;
    },

    mouseevent_set_screen_x: function(i, v) {
      let _i = A.g(i);
      _i.screenX = v;
    },

    mouseevent_get_screen_y: function(i) {
      let _i = A.g(i);
      return _i.screenY;
    },

    mouseevent_set_screen_y: function(i, v) {
      let _i = A.g(i);
      _i.screenY = v;
    },

    mouseevent_get_client_x: function(i) {
      let _i = A.g(i);
      return _i.clientX;
    },

    mouseevent_set_client_x: function(i, v) {
      let _i = A.g(i);
      _i.clientX = v;
    },

    mouseevent_get_client_y: function(i) {
      let _i = A.g(i);
      return _i.clientY;
    },

    mouseevent_set_client_y: function(i, v) {
      let _i = A.g(i);
      _i.clientY = v;
    },

    mouseevent_get_x: function(i) {
      let _i = A.g(i);
      return _i.x;
    },

    mouseevent_set_x: function(i, v) {
      let _i = A.g(i);
      _i.x = v;
    },

    mouseevent_get_y: function(i) {
      let _i = A.g(i);
      return _i.y;
    },

    mouseevent_set_y: function(i, v) {
      let _i = A.g(i);
      _i.y = v;
    },

    mouseevent_get_offset_x: function(i) {
      let _i = A.g(i);
      return _i.offsetX;
    },

    mouseevent_set_offset_x: function(i, v) {
      let _i = A.g(i);
      _i.offsetX = v;
    },

    mouseevent_get_offset_y: function(i) {
      let _i = A.g(i);
      return _i.offsetY;
    },

    mouseevent_set_offset_y: function(i, v) {
      let _i = A.g(i);
      _i.offsetY = v;
    },

    mouseevent_get_ctrl_key: function(i) {
      let _i = A.g(i);
      return _i.ctrlKey ? 1 : 0;
    },

    mouseevent_set_ctrl_key: function(i, v) {
      let _i = A.g(i);
      _i.ctrlKey = 1 == v;
    },

    mouseevent_get_shift_key: function(i) {
      let _i = A.g(i);
      return _i.shiftKey ? 1 : 0;
    },

    mouseevent_set_shift_key: function(i, v) {
      let _i = A.g(i);
      _i.shiftKey = 1 == v;
    },

    mouseevent_get_alt_key: function(i) {
      let _i = A.g(i);
      return _i.altKey ? 1 : 0;
    },

    mouseevent_set_alt_key: function(i, v) {
      let _i = A.g(i);
      _i.altKey = 1 == v;
    },

    mouseevent_get_meta_key: function(i) {
      let _i = A.g(i);
      return _i.metaKey ? 1 : 0;
    },

    mouseevent_set_meta_key: function(i, v) {
      let _i = A.g(i);
      _i.metaKey = 1 == v;
    },

    mouseevent_get_button: function(i) {
      let _i = A.g(i);
      return _i.button;
    },

    mouseevent_set_button: function(i, v) {
      let _i = A.g(i);
      _i.button = v;
    },

    mouseevent_get_buttons: function(i) {
      let _i = A.g(i);
      return _i.buttons;
    },

    mouseevent_set_buttons: function(i, v) {
      let _i = A.g(i);
      _i.buttons = v;
    },

    mouseevent_get_related_target: function(i) {
      let _i = A.g(i);
      return A.a(_i.relatedTarget);
    },

    mouseevent_set_related_target: function(i, v) {
      let _i = A.g(i);
      _i.relatedTarget = A.g(v);
    },

    mouseevent_get_region: function(i) {
      let _i = A.g(i);
      return this.ms(_i.region);
    },

    mouseevent_set_region: function(i, v) {
      let _i = A.g(i);
      _i.region = this.s(v);
    },

    mouseevent_get_movement_x: function(i) {
      let _i = A.g(i);
      return _i.movementX;
    },

    mouseevent_set_movement_x: function(i, v) {
      let _i = A.g(i);
      _i.movementX = v;
    },

    mouseevent_get_movement_y: function(i) {
      let _i = A.g(i);
      return _i.movementY;
    },

    mouseevent_set_movement_y: function(i, v) {
      let _i = A.g(i);
      _i.movementY = v;
    },

    mouseevent_init_mouse_event: function(
      i,
      typeArg,
      canBubbleArg,
      cancelableArg,
      viewArg,
      detailArg,
      screenXArg,
      screenYArg,
      clientXArg,
      clientYArg,
      ctrlKeyArg,
      altKeyArg,
      shiftKeyArg,
      metaKeyArg,
      buttonArg,
      relatedTargetArg
    ) {
      let _i = A.g(i);

      let _typeArg = this.s(typeArg);
      let _canBubbleArg = 0 != canBubbleArg;
      let _cancelableArg = 0 != cancelableArg;
      let _viewArg = A.g(viewArg);
      let _detailArg = detailArg;
      let _screenXArg = screenXArg;
      let _screenYArg = screenYArg;
      let _clientXArg = clientXArg;
      let _clientYArg = clientYArg;
      let _ctrlKeyArg = 0 != ctrlKeyArg;
      let _altKeyArg = 0 != altKeyArg;
      let _shiftKeyArg = 0 != shiftKeyArg;
      let _metaKeyArg = 0 != metaKeyArg;
      let _buttonArg = buttonArg;
      let _relatedTargetArg = A.g(relatedTargetArg);
      _i.initMouseEvent(
        _typeArg,
        _canBubbleArg,
        _cancelableArg,
        _viewArg,
        _detailArg,
        _screenXArg,
        _screenYArg,
        _clientXArg,
        _clientYArg,
        _ctrlKeyArg,
        _altKeyArg,
        _shiftKeyArg,
        _metaKeyArg,
        _buttonArg,
        _relatedTargetArg
      );
    },

    mouseevent_get_modifier_state: function(i, keyArg) {
      let _i = A.g(i);

      let _keyArg = this.s(keyArg);
      return A.a(_i.getModifierState(_keyArg)) ? 1 : 0;
    },

    node_get_node_type: function(i) {
      let _i = A.g(i);
      return _i.nodeType;
    },

    node_set_node_type: function(i, v) {
      let _i = A.g(i);
      _i.nodeType = v;
    },

    node_get_node_name: function(i) {
      let _i = A.g(i);
      return this.ms(_i.nodeName);
    },

    node_set_node_name: function(i, v) {
      let _i = A.g(i);
      _i.nodeName = this.s(v);
    },

    node_get_base_uri: function(i) {
      let _i = A.g(i);
      return this.ms(_i.baseURI);
    },

    node_set_base_uri: function(i, v) {
      let _i = A.g(i);
      _i.baseURI = this.s(v);
    },

    node_get_is_connected: function(i) {
      let _i = A.g(i);
      return _i.isConnected ? 1 : 0;
    },

    node_set_is_connected: function(i, v) {
      let _i = A.g(i);
      _i.isConnected = 1 == v;
    },

    node_get_owner_document: function(i) {
      let _i = A.g(i);
      return A.a(_i.ownerDocument);
    },

    node_set_owner_document: function(i, v) {
      let _i = A.g(i);
      _i.ownerDocument = A.g(v);
    },

    node_get_root_node: function(i, options) {
      let _i = A.g(i);

      let _options = A.g(options);
      return A.a(_i.getRootNode(_options));
    },

    node_get_parent_node: function(i) {
      let _i = A.g(i);
      return A.a(_i.parentNode);
    },

    node_set_parent_node: function(i, v) {
      let _i = A.g(i);
      _i.parentNode = A.g(v);
    },

    node_get_parent_element: function(i) {
      let _i = A.g(i);
      return A.a(_i.parentElement);
    },

    node_set_parent_element: function(i, v) {
      let _i = A.g(i);
      _i.parentElement = A.g(v);
    },

    node_has_child_nodes: function(i) {
      let _i = A.g(i);

      return A.a(_i.hasChildNodes()) ? 1 : 0;
    },

    node_get_child_nodes: function(i) {
      let _i = A.g(i);
      return A.a(_i.childNodes);
    },

    node_set_child_nodes: function(i, v) {
      let _i = A.g(i);
      _i.childNodes = A.g(v);
    },

    node_get_first_child: function(i) {
      let _i = A.g(i);
      return A.a(_i.firstChild);
    },

    node_set_first_child: function(i, v) {
      let _i = A.g(i);
      _i.firstChild = A.g(v);
    },

    node_get_last_child: function(i) {
      let _i = A.g(i);
      return A.a(_i.lastChild);
    },

    node_set_last_child: function(i, v) {
      let _i = A.g(i);
      _i.lastChild = A.g(v);
    },

    node_get_previous_sibling: function(i) {
      let _i = A.g(i);
      return A.a(_i.previousSibling);
    },

    node_set_previous_sibling: function(i, v) {
      let _i = A.g(i);
      _i.previousSibling = A.g(v);
    },

    node_get_next_sibling: function(i) {
      let _i = A.g(i);
      return A.a(_i.nextSibling);
    },

    node_set_next_sibling: function(i, v) {
      let _i = A.g(i);
      _i.nextSibling = A.g(v);
    },

    node_get_node_value: function(i) {
      let _i = A.g(i);
      return this.ms(_i.nodeValue);
    },

    node_set_node_value: function(i, v) {
      let _i = A.g(i);
      _i.nodeValue = this.s(v);
    },

    node_get_text_content: function(i) {
      let _i = A.g(i);
      return this.ms(_i.textContent);
    },

    node_set_text_content: function(i, v) {
      let _i = A.g(i);
      _i.textContent = this.s(v);
    },

    node_insert_before: function(i, node, child) {
      let _i = A.g(i);

      let _node = A.g(node);
      let _child = A.g(child);
      return A.a(_i.insertBefore(_node, _child));
    },

    node_append_child: function(i, node) {
      let _i = A.g(i);

      let _node = A.g(node);
      return A.a(_i.appendChild(_node));
    },

    node_replace_child: function(i, node, child) {
      let _i = A.g(i);

      let _node = A.g(node);
      let _child = A.g(child);
      return A.a(_i.replaceChild(_node, _child));
    },

    node_remove_child: function(i, child) {
      let _i = A.g(i);

      let _child = A.g(child);
      return A.a(_i.removeChild(_child));
    },

    node_normalize: function(i) {
      let _i = A.g(i);

      _i.normalize();
    },

    node_clone_node: function(i, deep) {
      let _i = A.g(i);

      let _deep = 0 != deep;
      return A.a(_i.cloneNode(_deep));
    },

    node_is_same_node: function(i, node) {
      let _i = A.g(i);

      let _node = A.g(node);
      return A.a(_i.isSameNode(_node)) ? 1 : 0;
    },

    node_is_equal_node: function(i, node) {
      let _i = A.g(i);

      let _node = A.g(node);
      return A.a(_i.isEqualNode(_node)) ? 1 : 0;
    },

    node_compare_document_position: function(i, other) {
      let _i = A.g(i);

      let _other = A.g(other);
      return A.a(_i.compareDocumentPosition(_other));
    },

    node_contains: function(i, other) {
      let _i = A.g(i);

      let _other = A.g(other);
      return A.a(_i.contains(_other)) ? 1 : 0;
    },

    node_lookup_prefix: function(i, namespace) {
      let _i = A.g(i);

      let _namespace = this.s(namespace);
      return this.ms(_i.lookupPrefix(_namespace));
    },

    node_lookup_namespace_uri: function(i, prefix) {
      let _i = A.g(i);

      let _prefix = this.s(prefix);
      return this.ms(_i.lookupNamespaceURI(_prefix));
    },

    node_is_default_namespace: function(i, namespace) {
      let _i = A.g(i);

      let _namespace = this.s(namespace);
      return A.a(_i.isDefaultNamespace(_namespace)) ? 1 : 0;
    },

    storage_get_length: function(i) {
      let _i = A.g(i);
      return _i.length;
    },

    storage_set_length: function(i, v) {
      let _i = A.g(i);
      _i.length = v;
    },

    storage_key: function(i, index) {
      let _i = A.g(i);

      let _index = index;
      return this.ms(_i.key(_index));
    },

    storage_get_item: function(i, key) {
      let _i = A.g(i);

      let _key = this.s(key);
      return this.ms(_i.getItem(_key));
    },

    storage_set_item: function(i, key, value) {
      let _i = A.g(i);

      let _key = this.s(key);
      let _value = this.s(value);
      _i.setItem(_key, _value);
    },

    storage_remove_item: function(i, key) {
      let _i = A.g(i);

      let _key = this.s(key);
      _i.removeItem(_key);
    },

    storage_clear: function(i) {
      let _i = A.g(i);

      _i.clear();
    },

    storage_get_is_session_only: function(i) {
      let _i = A.g(i);
      return _i.isSessionOnly ? 1 : 0;
    },

    storage_set_is_session_only: function(i, v) {
      let _i = A.g(i);
      _i.isSessionOnly = 1 == v;
    },

    webglactiveinfo_get_size: function(i) {
      let _i = A.g(i);
      return _i.size;
    },

    webglactiveinfo_set_size: function(i, v) {
      let _i = A.g(i);
      _i.size = v;
    },

    webglactiveinfo_get_type: function(i) {
      let _i = A.g(i);
      return _i.type;
    },

    webglactiveinfo_set_type: function(i, v) {
      let _i = A.g(i);
      _i.type = v;
    },

    webglactiveinfo_get_name: function(i) {
      let _i = A.g(i);
      return this.ms(_i.name);
    },

    webglactiveinfo_set_name: function(i, v) {
      let _i = A.g(i);
      _i.name = this.s(v);
    },

    webglshaderprecisionformat_get_range_min: function(i) {
      let _i = A.g(i);
      return _i.rangeMin;
    },

    webglshaderprecisionformat_set_range_min: function(i, v) {
      let _i = A.g(i);
      _i.rangeMin = v;
    },

    webglshaderprecisionformat_get_range_max: function(i) {
      let _i = A.g(i);
      return _i.rangeMax;
    },

    webglshaderprecisionformat_set_range_max: function(i, v) {
      let _i = A.g(i);
      _i.rangeMax = v;
    },

    webglshaderprecisionformat_get_precision: function(i) {
      let _i = A.g(i);
      return _i.precision;
    },

    webglshaderprecisionformat_set_precision: function(i, v) {
      let _i = A.g(i);
      _i.precision = v;
    },

    webgl_get_canvas: function(i) {
      let _i = A.g(i);
      return A.a(_i.canvas);
    },

    webgl_set_canvas: function(i, v) {
      let _i = A.g(i);
      _i.canvas = A.g(v);
    },

    webgl_get_drawing_buffer_width: function(i) {
      let _i = A.g(i);
      return _i.drawingBufferWidth;
    },

    webgl_set_drawing_buffer_width: function(i, v) {
      let _i = A.g(i);
      _i.drawingBufferWidth = v;
    },

    webgl_get_drawing_buffer_height: function(i) {
      let _i = A.g(i);
      return _i.drawingBufferHeight;
    },

    webgl_set_drawing_buffer_height: function(i, v) {
      let _i = A.g(i);
      _i.drawingBufferHeight = v;
    },

    webgl_get_context_attributes: function(i) {
      let _i = A.g(i);

      return A.a(_i.getContextAttributes());
    },

    webgl_is_context_lost: function(i) {
      let _i = A.g(i);

      return A.a(_i.isContextLost()) ? 1 : 0;
    },

    webgl_get_supported_extensions: function(i) {
      let _i = A.g(i);

      return A.a(_i.getSupportedExtensions());
    },

    webgl_get_extension: function(i, name) {
      let _i = A.g(i);

      let _name = this.s(name);
      return A.a(_i.getExtension(_name));
    },

    webgl_active_texture: function(i, texture) {
      let _i = A.g(i);

      let _texture = texture;
      _i.activeTexture(_texture);
    },

    webgl_attach_shader: function(i, program, shader) {
      let _i = A.g(i);

      let _program = A.g(program);
      let _shader = A.g(shader);
      _i.attachShader(_program, _shader);
    },

    webgl_bind_attrib_location: function(i, program, index, name) {
      let _i = A.g(i);

      let _program = A.g(program);
      let _index = index;
      let _name = this.s(name);
      _i.bindAttribLocation(_program, _index, _name);
    },

    webgl_bind_buffer: function(i, target, buffer) {
      let _i = A.g(i);

      let _target = target;
      let _buffer = A.g(buffer);
      _i.bindBuffer(_target, _buffer);
    },

    webgl_bind_framebuffer: function(i, target, framebuffer) {
      let _i = A.g(i);

      let _target = target;
      let _framebuffer = A.g(framebuffer);
      _i.bindFramebuffer(_target, _framebuffer);
    },

    webgl_bind_renderbuffer: function(i, target, renderbuffer) {
      let _i = A.g(i);

      let _target = target;
      let _renderbuffer = A.g(renderbuffer);
      _i.bindRenderbuffer(_target, _renderbuffer);
    },

    webgl_bind_texture: function(i, target, texture) {
      let _i = A.g(i);

      let _target = target;
      let _texture = A.g(texture);
      _i.bindTexture(_target, _texture);
    },

    webgl_blend_color: function(i, red, green, blue, alpha) {
      let _i = A.g(i);

      let _red = red;
      let _green = green;
      let _blue = blue;
      let _alpha = alpha;
      _i.blendColor(_red, _green, _blue, _alpha);
    },

    webgl_blend_equation: function(i, mode) {
      let _i = A.g(i);

      let _mode = mode;
      _i.blendEquation(_mode);
    },

    webgl_blend_equation_separate: function(i, modeRGB, modeAlpha) {
      let _i = A.g(i);

      let _modeRGB = modeRGB;
      let _modeAlpha = modeAlpha;
      _i.blendEquationSeparate(_modeRGB, _modeAlpha);
    },

    webgl_blend_func: function(i, sfactor, dfactor) {
      let _i = A.g(i);

      let _sfactor = sfactor;
      let _dfactor = dfactor;
      _i.blendFunc(_sfactor, _dfactor);
    },

    webgl_blend_func_separate: function(i, srcRGB, dstRGB, srcAlpha, dstAlpha) {
      let _i = A.g(i);

      let _srcRGB = srcRGB;
      let _dstRGB = dstRGB;
      let _srcAlpha = srcAlpha;
      let _dstAlpha = dstAlpha;
      _i.blendFuncSeparate(_srcRGB, _dstRGB, _srcAlpha, _dstAlpha);
    },

    webgl_check_framebuffer_status: function(i, target) {
      let _i = A.g(i);

      let _target = target;
      return A.a(_i.checkFramebufferStatus(_target));
    },

    webgl_clear: function(i, mask) {
      let _i = A.g(i);

      let _mask = mask;
      _i.clear(_mask);
    },

    webgl_clear_color: function(i, red, green, blue, alpha) {
      let _i = A.g(i);

      let _red = red;
      let _green = green;
      let _blue = blue;
      let _alpha = alpha;
      _i.clearColor(_red, _green, _blue, _alpha);
    },

    webgl_clear_depth: function(i, depth) {
      let _i = A.g(i);

      let _depth = depth;
      _i.clearDepth(_depth);
    },

    webgl_clear_stencil: function(i, s) {
      let _i = A.g(i);

      let _s = s;
      _i.clearStencil(_s);
    },

    webgl_color_mask: function(i, red, green, blue, alpha) {
      let _i = A.g(i);

      let _red = 0 != red;
      let _green = 0 != green;
      let _blue = 0 != blue;
      let _alpha = 0 != alpha;
      _i.colorMask(_red, _green, _blue, _alpha);
    },

    webgl_compile_shader: function(i, shader) {
      let _i = A.g(i);

      let _shader = A.g(shader);
      _i.compileShader(_shader);
    },

    webgl_copy_tex_image2_d: function(
      i,
      target,
      level,
      internalformat,
      x,
      y,
      width,
      height,
      border
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _internalformat = internalformat;
      let _x = x;
      let _y = y;
      let _width = width;
      let _height = height;
      let _border = border;
      _i.copyTexImage2D(
        _target,
        _level,
        _internalformat,
        _x,
        _y,
        _width,
        _height,
        _border
      );
    },

    webgl_copy_tex_sub_image2_d: function(
      i,
      target,
      level,
      xoffset,
      yoffset,
      x,
      y,
      width,
      height
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _xoffset = xoffset;
      let _yoffset = yoffset;
      let _x = x;
      let _y = y;
      let _width = width;
      let _height = height;
      _i.copyTexSubImage2D(
        _target,
        _level,
        _xoffset,
        _yoffset,
        _x,
        _y,
        _width,
        _height
      );
    },

    webgl_create_buffer: function(i) {
      let _i = A.g(i);

      return A.a(_i.createBuffer());
    },

    webgl_create_framebuffer: function(i) {
      let _i = A.g(i);

      return A.a(_i.createFramebuffer());
    },

    webgl_create_program: function(i) {
      let _i = A.g(i);

      return A.a(_i.createProgram());
    },

    webgl_create_renderbuffer: function(i) {
      let _i = A.g(i);

      return A.a(_i.createRenderbuffer());
    },

    webgl_create_shader: function(i, shaderType) {
      let _i = A.g(i);

      let _shaderType = shaderType;
      return A.a(_i.createShader(_shaderType));
    },

    webgl_create_texture: function(i) {
      let _i = A.g(i);

      return A.a(_i.createTexture());
    },

    webgl_cull_face: function(i, mode) {
      let _i = A.g(i);

      let _mode = mode;
      _i.cullFace(_mode);
    },

    webgl_delete_buffer: function(i, buffer) {
      let _i = A.g(i);

      let _buffer = A.g(buffer);
      _i.deleteBuffer(_buffer);
    },

    webgl_delete_framebuffer: function(i, framebuffer) {
      let _i = A.g(i);

      let _framebuffer = A.g(framebuffer);
      _i.deleteFramebuffer(_framebuffer);
    },

    webgl_delete_program: function(i, program) {
      let _i = A.g(i);

      let _program = A.g(program);
      _i.deleteProgram(_program);
    },

    webgl_delete_renderbuffer: function(i, renderbuffer) {
      let _i = A.g(i);

      let _renderbuffer = A.g(renderbuffer);
      _i.deleteRenderbuffer(_renderbuffer);
    },

    webgl_delete_shader: function(i, shader) {
      let _i = A.g(i);

      let _shader = A.g(shader);
      _i.deleteShader(_shader);
    },

    webgl_delete_texture: function(i, texture) {
      let _i = A.g(i);

      let _texture = A.g(texture);
      _i.deleteTexture(_texture);
    },

    webgl_depth_func: function(i, func) {
      let _i = A.g(i);

      let _func = func;
      _i.depthFunc(_func);
    },

    webgl_depth_mask: function(i, flag) {
      let _i = A.g(i);

      let _flag = 0 != flag;
      _i.depthMask(_flag);
    },

    webgl_depth_range: function(i, zNear, zFar) {
      let _i = A.g(i);

      let _zNear = zNear;
      let _zFar = zFar;
      _i.depthRange(_zNear, _zFar);
    },

    webgl_detach_shader: function(i, program, shader) {
      let _i = A.g(i);

      let _program = A.g(program);
      let _shader = A.g(shader);
      _i.detachShader(_program, _shader);
    },

    webgl_disable: function(i, cap) {
      let _i = A.g(i);

      let _cap = cap;
      _i.disable(_cap);
    },

    webgl_disable_vertex_attrib_array: function(i, index) {
      let _i = A.g(i);

      let _index = index;
      _i.disableVertexAttribArray(_index);
    },

    webgl_draw_arrays: function(i, mode, first, count) {
      let _i = A.g(i);

      let _mode = mode;
      let _first = first;
      let _count = count;
      _i.drawArrays(_mode, _first, _count);
    },

    webgl_draw_elements: function(i, mode, count, elementType, offset) {
      let _i = A.g(i);

      let _mode = mode;
      let _count = count;
      let _elementType = elementType;
      let _offset = offset;
      _i.drawElements(_mode, _count, _elementType, _offset);
    },

    webgl_enable: function(i, cap) {
      let _i = A.g(i);

      let _cap = cap;
      _i.enable(_cap);
    },

    webgl_enable_vertex_attrib_array: function(i, index) {
      let _i = A.g(i);

      let _index = index;
      _i.enableVertexAttribArray(_index);
    },

    webgl_finish: function(i) {
      let _i = A.g(i);

      _i.finish();
    },

    webgl_flush: function(i) {
      let _i = A.g(i);

      _i.flush();
    },

    webgl_framebuffer_renderbuffer: function(
      i,
      target,
      attachment,
      renderbuffertarget,
      renderbuffer
    ) {
      let _i = A.g(i);

      let _target = target;
      let _attachment = attachment;
      let _renderbuffertarget = renderbuffertarget;
      let _renderbuffer = A.g(renderbuffer);
      _i.framebufferRenderbuffer(
        _target,
        _attachment,
        _renderbuffertarget,
        _renderbuffer
      );
    },

    webgl_framebuffer_texture2_d: function(
      i,
      target,
      attachment,
      textarget,
      texture,
      level
    ) {
      let _i = A.g(i);

      let _target = target;
      let _attachment = attachment;
      let _textarget = textarget;
      let _texture = A.g(texture);
      let _level = level;
      _i.framebufferTexture2D(
        _target,
        _attachment,
        _textarget,
        _texture,
        _level
      );
    },

    webgl_front_face: function(i, mode) {
      let _i = A.g(i);

      let _mode = mode;
      _i.frontFace(_mode);
    },

    webgl_generate_mipmap: function(i, target) {
      let _i = A.g(i);

      let _target = target;
      _i.generateMipmap(_target);
    },

    webgl_get_active_attrib: function(i, program, index) {
      let _i = A.g(i);

      let _program = A.g(program);
      let _index = index;
      return A.a(_i.getActiveAttrib(_program, _index));
    },

    webgl_get_active_uniform: function(i, program, index) {
      let _i = A.g(i);

      let _program = A.g(program);
      let _index = index;
      return A.a(_i.getActiveUniform(_program, _index));
    },

    webgl_get_attached_shaders: function(i, program) {
      let _i = A.g(i);

      let _program = A.g(program);
      return A.a(_i.getAttachedShaders(_program));
    },

    webgl_get_attrib_location: function(i, program, name) {
      let _i = A.g(i);

      let _program = A.g(program);
      let _name = this.s(name);
      return A.a(_i.getAttribLocation(_program, _name));
    },

    webgl_get_buffer_parameter: function(i, target, pname) {
      let _i = A.g(i);

      let _target = target;
      let _pname = pname;
      return A.a(_i.getBufferParameter(_target, _pname));
    },

    webgl_get_parameter: function(i, pname) {
      let _i = A.g(i);

      let _pname = pname;
      return A.a(_i.getParameter(_pname));
    },

    webgl_get_error: function(i) {
      let _i = A.g(i);

      return A.a(_i.getError());
    },

    webgl_get_framebuffer_attachment_parameter: function(
      i,
      target,
      attachment,
      pname
    ) {
      let _i = A.g(i);

      let _target = target;
      let _attachment = attachment;
      let _pname = pname;
      return A.a(
        _i.getFramebufferAttachmentParameter(_target, _attachment, _pname)
      );
    },

    webgl_get_program_parameter: function(i, program, pname) {
      let _i = A.g(i);

      let _program = A.g(program);
      let _pname = pname;
      return A.a(_i.getProgramParameter(_program, _pname));
    },

    webgl_get_program_info_log: function(i, program) {
      let _i = A.g(i);

      let _program = A.g(program);
      return this.ms(_i.getProgramInfoLog(_program));
    },

    webgl_get_renderbuffer_parameter: function(i, target, pname) {
      let _i = A.g(i);

      let _target = target;
      let _pname = pname;
      return A.a(_i.getRenderbufferParameter(_target, _pname));
    },

    webgl_get_shader_parameter: function(i, shader, pname) {
      let _i = A.g(i);

      let _shader = A.g(shader);
      let _pname = pname;
      return A.a(_i.getShaderParameter(_shader, _pname));
    },

    webgl_get_shader_precision_format: function(i, shadertype, precisiontype) {
      let _i = A.g(i);

      let _shadertype = shadertype;
      let _precisiontype = precisiontype;
      return A.a(_i.getShaderPrecisionFormat(_shadertype, _precisiontype));
    },

    webgl_get_shader_info_log: function(i, shader) {
      let _i = A.g(i);

      let _shader = A.g(shader);
      return this.ms(_i.getShaderInfoLog(_shader));
    },

    webgl_get_shader_source: function(i, shader) {
      let _i = A.g(i);

      let _shader = A.g(shader);
      return this.ms(_i.getShaderSource(_shader));
    },

    webgl_get_tex_parameter: function(i, target, pname) {
      let _i = A.g(i);

      let _target = target;
      let _pname = pname;
      return A.a(_i.getTexParameter(_target, _pname));
    },

    webgl_get_uniform: function(i, program, location) {
      let _i = A.g(i);

      let _program = A.g(program);
      let _location = A.g(location);
      return A.a(_i.getUniform(_program, _location));
    },

    webgl_get_uniform_location: function(i, program, name) {
      let _i = A.g(i);

      let _program = A.g(program);
      let _name = this.s(name);
      return A.a(_i.getUniformLocation(_program, _name));
    },

    webgl_get_vertex_attrib: function(i, index, pname) {
      let _i = A.g(i);

      let _index = index;
      let _pname = pname;
      return A.a(_i.getVertexAttrib(_index, _pname));
    },

    webgl_get_vertex_attrib_offset: function(i, index, pname) {
      let _i = A.g(i);

      let _index = index;
      let _pname = pname;
      return A.a(_i.getVertexAttribOffset(_index, _pname));
    },

    webgl_hint: function(i, target, mode) {
      let _i = A.g(i);

      let _target = target;
      let _mode = mode;
      _i.hint(_target, _mode);
    },

    webgl_is_buffer: function(i, buffer) {
      let _i = A.g(i);

      let _buffer = A.g(buffer);
      return A.a(_i.isBuffer(_buffer)) ? 1 : 0;
    },

    webgl_is_enabled: function(i, cap) {
      let _i = A.g(i);

      let _cap = cap;
      return A.a(_i.isEnabled(_cap)) ? 1 : 0;
    },

    webgl_is_framebuffer: function(i, framebuffer) {
      let _i = A.g(i);

      let _framebuffer = A.g(framebuffer);
      return A.a(_i.isFramebuffer(_framebuffer)) ? 1 : 0;
    },

    webgl_is_program: function(i, program) {
      let _i = A.g(i);

      let _program = A.g(program);
      return A.a(_i.isProgram(_program)) ? 1 : 0;
    },

    webgl_is_renderbuffer: function(i, renderbuffer) {
      let _i = A.g(i);

      let _renderbuffer = A.g(renderbuffer);
      return A.a(_i.isRenderbuffer(_renderbuffer)) ? 1 : 0;
    },

    webgl_is_shader: function(i, shader) {
      let _i = A.g(i);

      let _shader = A.g(shader);
      return A.a(_i.isShader(_shader)) ? 1 : 0;
    },

    webgl_is_texture: function(i, texture) {
      let _i = A.g(i);

      let _texture = A.g(texture);
      return A.a(_i.isTexture(_texture)) ? 1 : 0;
    },

    webgl_line_width: function(i, width) {
      let _i = A.g(i);

      let _width = width;
      _i.lineWidth(_width);
    },

    webgl_link_program: function(i, program) {
      let _i = A.g(i);

      let _program = A.g(program);
      _i.linkProgram(_program);
    },

    webgl_pixel_storei: function(i, pname, param) {
      let _i = A.g(i);

      let _pname = pname;
      let _param = param;
      _i.pixelStorei(_pname, _param);
    },

    webgl_polygon_offset: function(i, factor, units) {
      let _i = A.g(i);

      let _factor = factor;
      let _units = units;
      _i.polygonOffset(_factor, _units);
    },

    webgl_renderbuffer_storage: function(
      i,
      target,
      internalformat,
      width,
      height
    ) {
      let _i = A.g(i);

      let _target = target;
      let _internalformat = internalformat;
      let _width = width;
      let _height = height;
      _i.renderbufferStorage(_target, _internalformat, _width, _height);
    },

    webgl_sample_coverage: function(i, value, invert) {
      let _i = A.g(i);

      let _value = value;
      let _invert = 0 != invert;
      _i.sampleCoverage(_value, _invert);
    },

    webgl_scissor: function(i, x, y, width, height) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      let _width = width;
      let _height = height;
      _i.scissor(_x, _y, _width, _height);
    },

    webgl_shader_source: function(i, shader, source) {
      let _i = A.g(i);

      let _shader = A.g(shader);
      let _source = this.s(source);
      _i.shaderSource(_shader, _source);
    },

    webgl_stencil_func: function(i, func, setencelRef, mask) {
      let _i = A.g(i);

      let _func = func;
      let _setencelRef = setencelRef;
      let _mask = mask;
      _i.stencilFunc(_func, _setencelRef, _mask);
    },

    webgl_stencil_func_separate: function(i, face, func, setencelRef, mask) {
      let _i = A.g(i);

      let _face = face;
      let _func = func;
      let _setencelRef = setencelRef;
      let _mask = mask;
      _i.stencilFuncSeparate(_face, _func, _setencelRef, _mask);
    },

    webgl_stencil_mask: function(i, mask) {
      let _i = A.g(i);

      let _mask = mask;
      _i.stencilMask(_mask);
    },

    webgl_stencil_mask_separate: function(i, face, mask) {
      let _i = A.g(i);

      let _face = face;
      let _mask = mask;
      _i.stencilMaskSeparate(_face, _mask);
    },

    webgl_stencil_op: function(i, fail, zfail, zpass) {
      let _i = A.g(i);

      let _fail = fail;
      let _zfail = zfail;
      let _zpass = zpass;
      _i.stencilOp(_fail, _zfail, _zpass);
    },

    webgl_stencil_op_separate: function(i, face, fail, zfail, zpass) {
      let _i = A.g(i);

      let _face = face;
      let _fail = fail;
      let _zfail = zfail;
      let _zpass = zpass;
      _i.stencilOpSeparate(_face, _fail, _zfail, _zpass);
    },

    webgl_tex_parameterf: function(i, target, pname, param) {
      let _i = A.g(i);

      let _target = target;
      let _pname = pname;
      let _param = param;
      _i.texParameterf(_target, _pname, _param);
    },

    webgl_tex_parameteri: function(i, target, pname, param) {
      let _i = A.g(i);

      let _target = target;
      let _pname = pname;
      let _param = param;
      _i.texParameteri(_target, _pname, _param);
    },

    webgl_uniform1f: function(i, location, x) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _x = x;
      _i.uniform1f(_location, _x);
    },

    webgl_uniform2f: function(i, location, x, y) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _x = x;
      let _y = y;
      _i.uniform2f(_location, _x, _y);
    },

    webgl_uniform3f: function(i, location, x, y, z) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _x = x;
      let _y = y;
      let _z = z;
      _i.uniform3f(_location, _x, _y, _z);
    },

    webgl_uniform4f: function(i, location, x, y, z, w) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _x = x;
      let _y = y;
      let _z = z;
      let _w = w;
      _i.uniform4f(_location, _x, _y, _z, _w);
    },

    webgl_uniform1i: function(i, location, x) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _x = x;
      _i.uniform1i(_location, _x);
    },

    webgl_uniform2i: function(i, location, x, y) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _x = x;
      let _y = y;
      _i.uniform2i(_location, _x, _y);
    },

    webgl_uniform3i: function(i, location, x, y, z) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _x = x;
      let _y = y;
      let _z = z;
      _i.uniform3i(_location, _x, _y, _z);
    },

    webgl_uniform4i: function(i, location, x, y, z, w) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _x = x;
      let _y = y;
      let _z = z;
      let _w = w;
      _i.uniform4i(_location, _x, _y, _z, _w);
    },

    webgl_use_program: function(i, program) {
      let _i = A.g(i);

      let _program = A.g(program);
      _i.useProgram(_program);
    },

    webgl_validate_program: function(i, program) {
      let _i = A.g(i);

      let _program = A.g(program);
      _i.validateProgram(_program);
    },

    webgl_vertex_attrib1f: function(i, indx, x) {
      let _i = A.g(i);

      let _indx = indx;
      let _x = x;
      _i.vertexAttrib1f(_indx, _x);
    },

    webgl_vertex_attrib1fv: function(i, indx, values) {
      let _i = A.g(i);

      let _indx = indx;
      let _values = A.g(values);
      _i.vertexAttrib1fv(_indx, _values);
    },

    webgl_vertex_attrib2f: function(i, indx, x, y) {
      let _i = A.g(i);

      let _indx = indx;
      let _x = x;
      let _y = y;
      _i.vertexAttrib2f(_indx, _x, _y);
    },

    webgl_vertex_attrib2fv: function(i, indx, values) {
      let _i = A.g(i);

      let _indx = indx;
      let _values = A.g(values);
      _i.vertexAttrib2fv(_indx, _values);
    },

    webgl_vertex_attrib3f: function(i, indx, x, y, z) {
      let _i = A.g(i);

      let _indx = indx;
      let _x = x;
      let _y = y;
      let _z = z;
      _i.vertexAttrib3f(_indx, _x, _y, _z);
    },

    webgl_vertex_attrib3fv: function(i, indx, values) {
      let _i = A.g(i);

      let _indx = indx;
      let _values = A.g(values);
      _i.vertexAttrib3fv(_indx, _values);
    },

    webgl_vertex_attrib4f: function(i, indx, x, y, z, w) {
      let _i = A.g(i);

      let _indx = indx;
      let _x = x;
      let _y = y;
      let _z = z;
      let _w = w;
      _i.vertexAttrib4f(_indx, _x, _y, _z, _w);
    },

    webgl_vertex_attrib4fv: function(i, indx, values) {
      let _i = A.g(i);

      let _indx = indx;
      let _values = A.g(values);
      _i.vertexAttrib4fv(_indx, _values);
    },

    webgl_vertex_attrib_pointer: function(
      i,
      indx,
      size,
      pointerType,
      normalized,
      stride,
      offset
    ) {
      let _i = A.g(i);

      let _indx = indx;
      let _size = size;
      let _pointerType = pointerType;
      let _normalized = 0 != normalized;
      let _stride = stride;
      let _offset = offset;
      _i.vertexAttribPointer(
        _indx,
        _size,
        _pointerType,
        _normalized,
        _stride,
        _offset
      );
    },

    webgl_viewport: function(i, x, y, width, height) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      let _width = width;
      let _height = height;
      _i.viewport(_x, _y, _width, _height);
    },

    webgl_buffer_data: function(i, target, data, usage) {
      let _i = A.g(i);

      let _target = target;
      let _data = A.g(data);
      let _usage = usage;
      _i.bufferData(_target, _data, _usage);
    },

    webgl_buffer_data_1: function(i, target, size, usage) {
      let _i = A.g(i);

      let _target = target;
      let _size = A.g(size);
      let _usage = usage;
      _i.bufferData(_target, _size, _usage);
    },

    webgl_buffer_data_2: function(i, target, data, usage) {
      let _i = A.g(i);

      let _target = target;
      let _data = A.g(data);
      let _usage = usage;
      _i.bufferData(_target, _data, _usage);
    },

    webgl_buffer_sub_data: function(i, target, offset, data) {
      let _i = A.g(i);

      let _target = target;
      let _offset = offset;
      let _data = A.g(data);
      _i.bufferSubData(_target, _offset, _data);
    },

    webgl_buffer_sub_data_1: function(i, target, offset, data) {
      let _i = A.g(i);

      let _target = target;
      let _offset = offset;
      let _data = A.g(data);
      _i.bufferSubData(_target, _offset, _data);
    },

    webgl_compressed_tex_image2_d: function(
      i,
      target,
      level,
      internalformat,
      width,
      height,
      border,
      data
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _internalformat = internalformat;
      let _width = width;
      let _height = height;
      let _border = border;
      let _data = A.g(data);
      _i.compressedTexImage2D(
        _target,
        _level,
        _internalformat,
        _width,
        _height,
        _border,
        _data
      );
    },

    webgl_compressed_tex_sub_image2_d: function(
      i,
      target,
      level,
      xoffset,
      yoffset,
      width,
      height,
      format,
      data
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _xoffset = xoffset;
      let _yoffset = yoffset;
      let _width = width;
      let _height = height;
      let _format = format;
      let _data = A.g(data);
      _i.compressedTexSubImage2D(
        _target,
        _level,
        _xoffset,
        _yoffset,
        _width,
        _height,
        _format,
        _data
      );
    },

    webgl_read_pixels: function(
      i,
      x,
      y,
      width,
      height,
      format,
      pixelType,
      pixels
    ) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      let _width = width;
      let _height = height;
      let _format = format;
      let _pixelType = pixelType;
      let _pixels = A.g(pixels);
      _i.readPixels(_x, _y, _width, _height, _format, _pixelType, _pixels);
    },

    webgl_tex_image2_d: function(
      i,
      target,
      level,
      internalformat,
      width,
      height,
      border,
      format,
      imageType,
      pixels
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _internalformat = internalformat;
      let _width = width;
      let _height = height;
      let _border = border;
      let _format = format;
      let _imageType = imageType;
      let _pixels = A.g(pixels);
      _i.texImage2D(
        _target,
        _level,
        _internalformat,
        _width,
        _height,
        _border,
        _format,
        _imageType,
        _pixels
      );
    },

    webgl_tex_image2_d_1: function(
      i,
      target,
      level,
      internalformat,
      format,
      imageType,
      pixels
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _internalformat = internalformat;
      let _format = format;
      let _imageType = imageType;
      let _pixels = A.g(pixels);
      _i.texImage2D(
        _target,
        _level,
        _internalformat,
        _format,
        _imageType,
        _pixels
      );
    },

    webgl_tex_image2_d_2: function(
      i,
      target,
      level,
      internalformat,
      format,
      imageType,
      pixels
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _internalformat = internalformat;
      let _format = format;
      let _imageType = imageType;
      let _pixels = A.g(pixels);
      _i.texImage2D(
        _target,
        _level,
        _internalformat,
        _format,
        _imageType,
        _pixels
      );
    },

    webgl_tex_image2_d_3: function(
      i,
      target,
      level,
      internalformat,
      format,
      imageType,
      image
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _internalformat = internalformat;
      let _format = format;
      let _imageType = imageType;
      let _image = A.g(image);
      _i.texImage2D(
        _target,
        _level,
        _internalformat,
        _format,
        _imageType,
        _image
      );
    },

    webgl_tex_image2_d_4: function(
      i,
      target,
      level,
      internalformat,
      format,
      imageType,
      canvas
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _internalformat = internalformat;
      let _format = format;
      let _imageType = imageType;
      let _canvas = A.g(canvas);
      _i.texImage2D(
        _target,
        _level,
        _internalformat,
        _format,
        _imageType,
        _canvas
      );
    },

    webgl_tex_image2_d_5: function(
      i,
      target,
      level,
      internalformat,
      format,
      imageType,
      video
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _internalformat = internalformat;
      let _format = format;
      let _imageType = imageType;
      let _video = A.g(video);
      _i.texImage2D(
        _target,
        _level,
        _internalformat,
        _format,
        _imageType,
        _video
      );
    },

    webgl_tex_sub_image2_d: function(
      i,
      target,
      level,
      xoffset,
      yoffset,
      width,
      height,
      format,
      imageType,
      pixels
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _xoffset = xoffset;
      let _yoffset = yoffset;
      let _width = width;
      let _height = height;
      let _format = format;
      let _imageType = imageType;
      let _pixels = A.g(pixels);
      _i.texSubImage2D(
        _target,
        _level,
        _xoffset,
        _yoffset,
        _width,
        _height,
        _format,
        _imageType,
        _pixels
      );
    },

    webgl_tex_sub_image2_d_1: function(
      i,
      target,
      level,
      xoffset,
      yoffset,
      format,
      imageType,
      pixels
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _xoffset = xoffset;
      let _yoffset = yoffset;
      let _format = format;
      let _imageType = imageType;
      let _pixels = A.g(pixels);
      _i.texSubImage2D(
        _target,
        _level,
        _xoffset,
        _yoffset,
        _format,
        _imageType,
        _pixels
      );
    },

    webgl_tex_sub_image2_d_2: function(
      i,
      target,
      level,
      xoffset,
      yoffset,
      format,
      imageType,
      pixels
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _xoffset = xoffset;
      let _yoffset = yoffset;
      let _format = format;
      let _imageType = imageType;
      let _pixels = A.g(pixels);
      _i.texSubImage2D(
        _target,
        _level,
        _xoffset,
        _yoffset,
        _format,
        _imageType,
        _pixels
      );
    },

    webgl_tex_sub_image2_d_3: function(
      i,
      target,
      level,
      xoffset,
      yoffset,
      format,
      imageType,
      image
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _xoffset = xoffset;
      let _yoffset = yoffset;
      let _format = format;
      let _imageType = imageType;
      let _image = A.g(image);
      _i.texSubImage2D(
        _target,
        _level,
        _xoffset,
        _yoffset,
        _format,
        _imageType,
        _image
      );
    },

    webgl_tex_sub_image2_d_4: function(
      i,
      target,
      level,
      xoffset,
      yoffset,
      format,
      imageType,
      canvas
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _xoffset = xoffset;
      let _yoffset = yoffset;
      let _format = format;
      let _imageType = imageType;
      let _canvas = A.g(canvas);
      _i.texSubImage2D(
        _target,
        _level,
        _xoffset,
        _yoffset,
        _format,
        _imageType,
        _canvas
      );
    },

    webgl_tex_sub_image2_d_5: function(
      i,
      target,
      level,
      xoffset,
      yoffset,
      format,
      imageType,
      video
    ) {
      let _i = A.g(i);

      let _target = target;
      let _level = level;
      let _xoffset = xoffset;
      let _yoffset = yoffset;
      let _format = format;
      let _imageType = imageType;
      let _video = A.g(video);
      _i.texSubImage2D(
        _target,
        _level,
        _xoffset,
        _yoffset,
        _format,
        _imageType,
        _video
      );
    },

    webgl_uniform1fv: function(i, location, data) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _data = A.g(data);
      _i.uniform1fv(_location, _data);
    },

    webgl_uniform2fv: function(i, location, data) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _data = A.g(data);
      _i.uniform2fv(_location, _data);
    },

    webgl_uniform3fv: function(i, location, data) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _data = A.g(data);
      _i.uniform3fv(_location, _data);
    },

    webgl_uniform4fv: function(i, location, data) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _data = A.g(data);
      _i.uniform4fv(_location, _data);
    },

    webgl_uniform1iv: function(i, location, data) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _data = A.g(data);
      _i.uniform1iv(_location, _data);
    },

    webgl_uniform2iv: function(i, location, data) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _data = A.g(data);
      _i.uniform2iv(_location, _data);
    },

    webgl_uniform3iv: function(i, location, data) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _data = A.g(data);
      _i.uniform3iv(_location, _data);
    },

    webgl_uniform4iv: function(i, location, data) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _data = A.g(data);
      _i.uniform4iv(_location, _data);
    },

    webgl_uniform_matrix2fv: function(i, location, transpose, data) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _transpose = 0 != transpose;
      let _data = A.g(data);
      _i.uniformMatrix2fv(_location, _transpose, _data);
    },

    webgl_uniform_matrix3fv: function(i, location, transpose, data) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _transpose = 0 != transpose;
      let _data = A.g(data);
      _i.uniformMatrix3fv(_location, _transpose, _data);
    },

    webgl_uniform_matrix4fv: function(i, location, transpose, data) {
      let _i = A.g(i);

      let _location = A.g(location);
      let _transpose = 0 != transpose;
      let _data = A.g(data);
      _i.uniformMatrix4fv(_location, _transpose, _data);
    },

    webgl_commit: function(i) {
      let _i = A.g(i);

      _i.commit();
    },

    webglcompressedtextureastc_get_supported_profiles: function(i) {
      let _i = A.g(i);

      return A.a(_i.getSupportedProfiles());
    },

    webgldebugshaders_get_translated_shader_source: function(i, shader) {
      let _i = A.g(i);

      let _shader = A.g(shader);
      return this.ms(_i.getTranslatedShaderSource(_shader));
    },

    webgllosecontext_lose_context: function(i) {
      let _i = A.g(i);

      _i.loseContext();
    },

    webgllosecontext_restore_context: function(i) {
      let _i = A.g(i);

      _i.restoreContext();
    },

    webgldrawbuffers_draw_buffers_w_e_b_g_l: function(i, buffers) {
      let _i = A.g(i);

      let _buffers = A.g(buffers);
      _i.drawBuffersWEBGL(_buffers);
    },

    oesvertexarrayobject_create_vertex_array_o_e_s: function(i) {
      let _i = A.g(i);

      return A.a(_i.createVertexArrayOES());
    },

    oesvertexarrayobject_delete_vertex_array_o_e_s: function(i, arrayObject) {
      let _i = A.g(i);

      let _arrayObject = A.g(arrayObject);
      _i.deleteVertexArrayOES(_arrayObject);
    },

    oesvertexarrayobject_is_vertex_array_o_e_s: function(i, arrayObject) {
      let _i = A.g(i);

      let _arrayObject = A.g(arrayObject);
      return A.a(_i.isVertexArrayOES(_arrayObject)) ? 1 : 0;
    },

    oesvertexarrayobject_bind_vertex_array_o_e_s: function(i, arrayObject) {
      let _i = A.g(i);

      let _arrayObject = A.g(arrayObject);
      _i.bindVertexArrayOES(_arrayObject);
    },

    angleinstancedarrays_draw_arrays_instanced_angle: function(
      i,
      mode,
      first,
      count,
      primcount
    ) {
      let _i = A.g(i);

      let _mode = mode;
      let _first = first;
      let _count = count;
      let _primcount = primcount;
      _i.drawArraysInstancedANGLE(_mode, _first, _count, _primcount);
    },

    angleinstancedarrays_draw_elements_instanced_angle: function(
      i,
      mode,
      count,
      elementType,
      offset,
      primcount
    ) {
      let _i = A.g(i);

      let _mode = mode;
      let _count = count;
      let _elementType = elementType;
      let _offset = offset;
      let _primcount = primcount;
      _i.drawElementsInstancedANGLE(
        _mode,
        _count,
        _elementType,
        _offset,
        _primcount
      );
    },

    angleinstancedarrays_vertex_attrib_divisor_angle: function(
      i,
      index,
      divisor
    ) {
      let _i = A.g(i);

      let _index = index;
      let _divisor = divisor;
      _i.vertexAttribDivisorANGLE(_index, _divisor);
    },

    extdisjointtimerquery_create_query_e_x_t: function(i) {
      let _i = A.g(i);

      return A.a(_i.createQueryEXT());
    },

    extdisjointtimerquery_delete_query_e_x_t: function(i, query) {
      let _i = A.g(i);

      let _query = A.g(query);
      _i.deleteQueryEXT(_query);
    },

    extdisjointtimerquery_is_query_e_x_t: function(i, query) {
      let _i = A.g(i);

      let _query = A.g(query);
      return A.a(_i.isQueryEXT(_query)) ? 1 : 0;
    },

    extdisjointtimerquery_begin_query_e_x_t: function(i, target, query) {
      let _i = A.g(i);

      let _target = target;
      let _query = A.g(query);
      _i.beginQueryEXT(_target, _query);
    },

    extdisjointtimerquery_end_query_e_x_t: function(i, target) {
      let _i = A.g(i);

      let _target = target;
      _i.endQueryEXT(_target);
    },

    extdisjointtimerquery_query_counter_e_x_t: function(i, query, target) {
      let _i = A.g(i);

      let _query = A.g(query);
      let _target = target;
      _i.queryCounterEXT(_query, _target);
    },

    extdisjointtimerquery_get_query_e_x_t: function(i, target, pname) {
      let _i = A.g(i);

      let _target = target;
      let _pname = pname;
      return A.a(_i.getQueryEXT(_target, _pname));
    },

    extdisjointtimerquery_get_query_object_e_x_t: function(i, query, pname) {
      let _i = A.g(i);

      let _query = A.g(query);
      let _pname = pname;
      return A.a(_i.getQueryObjectEXT(_query, _pname));
    },

    window_get_window: function(i) {
      let _i = A.g(i);
      return A.a(_i.window);
    },

    window_set_window: function(i, v) {
      let _i = A.g(i);
      _i.window = A.g(v);
    },

    window_get_self: function(i) {
      let _i = A.g(i);
      return A.a(_i.self);
    },

    window_set_self: function(i, v) {
      let _i = A.g(i);
      _i.self = A.g(v);
    },

    window_get_document: function(i) {
      let _i = A.g(i);
      return A.a(_i.document);
    },

    window_set_document: function(i, v) {
      let _i = A.g(i);
      _i.document = A.g(v);
    },

    window_get_name: function(i) {
      let _i = A.g(i);
      return this.ms(_i.name);
    },

    window_set_name: function(i, v) {
      let _i = A.g(i);
      _i.name = this.s(v);
    },

    window_get_location: function(i) {
      let _i = A.g(i);
      return A.a(_i.location);
    },

    window_set_location: function(i, v) {
      let _i = A.g(i);
      _i.location = A.g(v);
    },

    window_get_history: function(i) {
      let _i = A.g(i);
      return A.a(_i.history);
    },

    window_set_history: function(i, v) {
      let _i = A.g(i);
      _i.history = A.g(v);
    },

    window_get_custom_elements: function(i) {
      let _i = A.g(i);
      return A.a(_i.customElements);
    },

    window_set_custom_elements: function(i, v) {
      let _i = A.g(i);
      _i.customElements = A.g(v);
    },

    window_get_locationbar: function(i) {
      let _i = A.g(i);
      return A.a(_i.locationbar);
    },

    window_set_locationbar: function(i, v) {
      let _i = A.g(i);
      _i.locationbar = A.g(v);
    },

    window_get_menubar: function(i) {
      let _i = A.g(i);
      return A.a(_i.menubar);
    },

    window_set_menubar: function(i, v) {
      let _i = A.g(i);
      _i.menubar = A.g(v);
    },

    window_get_personalbar: function(i) {
      let _i = A.g(i);
      return A.a(_i.personalbar);
    },

    window_set_personalbar: function(i, v) {
      let _i = A.g(i);
      _i.personalbar = A.g(v);
    },

    window_get_scrollbars: function(i) {
      let _i = A.g(i);
      return A.a(_i.scrollbars);
    },

    window_set_scrollbars: function(i, v) {
      let _i = A.g(i);
      _i.scrollbars = A.g(v);
    },

    window_get_statusbar: function(i) {
      let _i = A.g(i);
      return A.a(_i.statusbar);
    },

    window_set_statusbar: function(i, v) {
      let _i = A.g(i);
      _i.statusbar = A.g(v);
    },

    window_get_toolbar: function(i) {
      let _i = A.g(i);
      return A.a(_i.toolbar);
    },

    window_set_toolbar: function(i, v) {
      let _i = A.g(i);
      _i.toolbar = A.g(v);
    },

    window_get_status: function(i) {
      let _i = A.g(i);
      return this.ms(_i.status);
    },

    window_set_status: function(i, v) {
      let _i = A.g(i);
      _i.status = this.s(v);
    },

    window_close: function(i) {
      let _i = A.g(i);

      _i.close();
    },

    window_get_closed: function(i) {
      let _i = A.g(i);
      return _i.closed ? 1 : 0;
    },

    window_set_closed: function(i, v) {
      let _i = A.g(i);
      _i.closed = 1 == v;
    },

    window_stop: function(i) {
      let _i = A.g(i);

      _i.stop();
    },

    window_focus: function(i) {
      let _i = A.g(i);

      _i.focus();
    },

    window_blur: function(i) {
      let _i = A.g(i);

      _i.blur();
    },

    window_get_event: function(i) {
      let _i = A.g(i);
      return A.a(_i.event);
    },

    window_set_event: function(i, v) {
      let _i = A.g(i);
      _i.event = A.g(v);
    },

    window_get_frames: function(i) {
      let _i = A.g(i);
      return A.a(_i.frames);
    },

    window_set_frames: function(i, v) {
      let _i = A.g(i);
      _i.frames = A.g(v);
    },

    window_get_length: function(i) {
      let _i = A.g(i);
      return _i.length;
    },

    window_set_length: function(i, v) {
      let _i = A.g(i);
      _i.length = v;
    },

    window_get_top: function(i) {
      let _i = A.g(i);
      return A.a(_i.top);
    },

    window_set_top: function(i, v) {
      let _i = A.g(i);
      _i.top = A.g(v);
    },

    window_get_opener: function(i) {
      let _i = A.g(i);
      return A.a(_i.opener);
    },

    window_set_opener: function(i, v) {
      let _i = A.g(i);
      _i.opener = A.g(v);
    },

    window_get_parent: function(i) {
      let _i = A.g(i);
      return A.a(_i.parent);
    },

    window_set_parent: function(i, v) {
      let _i = A.g(i);
      _i.parent = A.g(v);
    },

    window_get_frame_element: function(i) {
      let _i = A.g(i);
      return A.a(_i.frameElement);
    },

    window_set_frame_element: function(i, v) {
      let _i = A.g(i);
      _i.frameElement = A.g(v);
    },

    window_open: function(i, url, target, features) {
      let _i = A.g(i);

      let _url = this.s(url);
      let _target = this.s(target);
      let _features = this.s(features);
      return A.a(_i.open(_url, _target, _features));
    },

    window_get_navigator: function(i) {
      let _i = A.g(i);
      return A.a(_i.navigator);
    },

    window_set_navigator: function(i, v) {
      let _i = A.g(i);
      _i.navigator = A.g(v);
    },

    window_get_external: function(i) {
      let _i = A.g(i);
      return A.a(_i.external);
    },

    window_set_external: function(i, v) {
      let _i = A.g(i);
      _i.external = A.g(v);
    },

    window_get_application_cache: function(i) {
      let _i = A.g(i);
      return A.a(_i.applicationCache);
    },

    window_set_application_cache: function(i, v) {
      let _i = A.g(i);
      _i.applicationCache = A.g(v);
    },

    window_alert: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      _i.alert(_message);
    },

    window_confirm: function(i, message) {
      let _i = A.g(i);

      let _message = this.s(message);
      return A.a(_i.confirm(_message)) ? 1 : 0;
    },

    window_prompt: function(i, message, defaultMessage) {
      let _i = A.g(i);

      let _message = this.s(message);
      let _defaultMessage = this.s(defaultMessage);
      return this.ms(_i.prompt(_message, _defaultMessage));
    },

    window_print: function(i) {
      let _i = A.g(i);

      _i.print();
    },

    window_post_message: function(i, message, targetOrigin, transfer) {
      let _i = A.g(i);

      let _message = A.g(message);
      let _targetOrigin = this.s(targetOrigin);
      let _transfer = A.g(transfer);
      _i.postMessage(_message, _targetOrigin, _transfer);
    },

    window_get_onappinstalled: function(i) {
      let _i = A.g(i);
      return A.a(_i.onappinstalled);
    },

    window_set_onappinstalled: function(i, v) {
      let _i = A.g(i);
      _i.onappinstalled = A.g(v);
    },

    window_get_session_storage: function(i) {
      let _i = A.g(i);
      return A.a(_i.sessionStorage);
    },

    window_set_session_storage: function(i, v) {
      let _i = A.g(i);
      _i.sessionStorage = A.g(v);
    },

    window_get_local_storage: function(i) {
      let _i = A.g(i);
      return A.a(_i.localStorage);
    },

    window_set_local_storage: function(i, v) {
      let _i = A.g(i);
      _i.localStorage = A.g(v);
    },

    window_capture_events: function(i) {
      let _i = A.g(i);

      _i.captureEvents();
    },

    window_release_events: function(i) {
      let _i = A.g(i);

      _i.releaseEvents();
    },

    window_get_selection: function(i) {
      let _i = A.g(i);

      return A.a(_i.getSelection());
    },

    window_get_computed_style: function(i, elt, pseudoElt) {
      let _i = A.g(i);

      let _elt = A.g(elt);
      let _pseudoElt = this.s(pseudoElt);
      return A.a(_i.getComputedStyle(_elt, _pseudoElt));
    },

    window_match_media: function(i, query) {
      let _i = A.g(i);

      let _query = this.s(query);
      return A.a(_i.matchMedia(_query));
    },

    window_get_screen: function(i) {
      let _i = A.g(i);
      return A.a(_i.screen);
    },

    window_set_screen: function(i, v) {
      let _i = A.g(i);
      _i.screen = A.g(v);
    },

    window_move_to: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.moveTo(_x, _y);
    },

    window_move_by: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.moveBy(_x, _y);
    },

    window_resize_to: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.resizeTo(_x, _y);
    },

    window_resize_by: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.resizeBy(_x, _y);
    },

    window_get_inner_width: function(i) {
      let _i = A.g(i);
      return A.a(_i.innerWidth);
    },

    window_set_inner_width: function(i, v) {
      let _i = A.g(i);
      _i.innerWidth = A.g(v);
    },

    window_get_inner_height: function(i) {
      let _i = A.g(i);
      return A.a(_i.innerHeight);
    },

    window_set_inner_height: function(i, v) {
      let _i = A.g(i);
      _i.innerHeight = A.g(v);
    },

    window_scroll: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.scroll(_x, _y);
    },

    window_scroll_to: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.scrollTo(_x, _y);
    },

    window_scroll_by: function(i, x, y) {
      let _i = A.g(i);

      let _x = x;
      let _y = y;
      _i.scrollBy(_x, _y);
    },

    window_get_scroll_x: function(i) {
      let _i = A.g(i);
      return _i.scrollX;
    },

    window_set_scroll_x: function(i, v) {
      let _i = A.g(i);
      _i.scrollX = v;
    },

    window_get_page_x_offset: function(i) {
      let _i = A.g(i);
      return _i.pageXOffset;
    },

    window_set_page_x_offset: function(i, v) {
      let _i = A.g(i);
      _i.pageXOffset = v;
    },

    window_get_scroll_y: function(i) {
      let _i = A.g(i);
      return _i.scrollY;
    },

    window_set_scroll_y: function(i, v) {
      let _i = A.g(i);
      _i.scrollY = v;
    },

    window_get_page_y_offset: function(i) {
      let _i = A.g(i);
      return _i.pageYOffset;
    },

    window_set_page_y_offset: function(i, v) {
      let _i = A.g(i);
      _i.pageYOffset = v;
    },

    window_get_screen_x: function(i) {
      let _i = A.g(i);
      return A.a(_i.screenX);
    },

    window_set_screen_x: function(i, v) {
      let _i = A.g(i);
      _i.screenX = A.g(v);
    },

    window_get_screen_y: function(i) {
      let _i = A.g(i);
      return A.a(_i.screenY);
    },

    window_set_screen_y: function(i, v) {
      let _i = A.g(i);
      _i.screenY = A.g(v);
    },

    window_get_outer_width: function(i) {
      let _i = A.g(i);
      return A.a(_i.outerWidth);
    },

    window_set_outer_width: function(i, v) {
      let _i = A.g(i);
      _i.outerWidth = A.g(v);
    },

    window_get_outer_height: function(i) {
      let _i = A.g(i);
      return A.a(_i.outerHeight);
    },

    window_set_outer_height: function(i, v) {
      let _i = A.g(i);
      _i.outerHeight = A.g(v);
    },

    window_get_device_pixel_ratio: function(i) {
      let _i = A.g(i);
      return _i.devicePixelRatio;
    },

    window_set_device_pixel_ratio: function(i, v) {
      let _i = A.g(i);
      _i.devicePixelRatio = v;
    },

    window_request_animation_frame: function(i, callback) {
      let _i = A.g(i);

      let _callback = A.g(callback);
      return A.a(_i.requestAnimationFrame(_callback));
    },

    window_cancel_animation_frame: function(i, handle) {
      let _i = A.g(i);

      let _handle = handle;
      _i.cancelAnimationFrame(_handle);
    },

    window_get_performance: function(i) {
      let _i = A.g(i);
      return A.a(_i.performance);
    },

    window_set_performance: function(i, v) {
      let _i = A.g(i);
      _i.performance = A.g(v);
    },

    window_get_orientation: function(i) {
      let _i = A.g(i);
      return _i.orientation;
    },

    window_set_orientation: function(i, v) {
      let _i = A.g(i);
      _i.orientation = v;
    },

    window_get_onorientationchange: function(i) {
      let _i = A.g(i);
      return A.a(_i.onorientationchange);
    },

    window_set_onorientationchange: function(i, v) {
      let _i = A.g(i);
      _i.onorientationchange = A.g(v);
    },

    window_get_onvrdisplayconnect: function(i) {
      let _i = A.g(i);
      return A.a(_i.onvrdisplayconnect);
    },

    window_set_onvrdisplayconnect: function(i, v) {
      let _i = A.g(i);
      _i.onvrdisplayconnect = A.g(v);
    },

    window_get_onvrdisplaydisconnect: function(i) {
      let _i = A.g(i);
      return A.a(_i.onvrdisplaydisconnect);
    },

    window_set_onvrdisplaydisconnect: function(i, v) {
      let _i = A.g(i);
      _i.onvrdisplaydisconnect = A.g(v);
    },

    window_get_onvrdisplayactivate: function(i) {
      let _i = A.g(i);
      return A.a(_i.onvrdisplayactivate);
    },

    window_set_onvrdisplayactivate: function(i, v) {
      let _i = A.g(i);
      _i.onvrdisplayactivate = A.g(v);
    },

    window_get_onvrdisplaydeactivate: function(i) {
      let _i = A.g(i);
      return A.a(_i.onvrdisplaydeactivate);
    },

    window_set_onvrdisplaydeactivate: function(i, v) {
      let _i = A.g(i);
      _i.onvrdisplaydeactivate = A.g(v);
    },

    window_get_onvrdisplaypresentchange: function(i) {
      let _i = A.g(i);
      return A.a(_i.onvrdisplaypresentchange);
    },

    window_set_onvrdisplaypresentchange: function(i, v) {
      let _i = A.g(i);
      _i.onvrdisplaypresentchange = A.g(v);
    },

    window_get_paint_worklet: function(i) {
      let _i = A.g(i);
      return A.a(_i.paintWorklet);
    },

    window_set_paint_worklet: function(i, v) {
      let _i = A.g(i);
      _i.paintWorklet = A.g(v);
    },

    window_request_idle_callback: function(i, callback, options) {
      let _i = A.g(i);

      let _callback = A.g(callback);
      let _options = A.g(options);
      return A.a(_i.requestIdleCallback(_callback, _options));
    },

    window_cancel_idle_callback: function(i, handle) {
      let _i = A.g(i);

      let _handle = handle;
      _i.cancelIdleCallback(_handle);
    },

    window_get_origin: function(i) {
      let _i = A.g(i);
      return A.a(_i.origin);
    },

    window_set_origin: function(i, v) {
      let _i = A.g(i);
      _i.origin = A.g(v);
    },

    window_btoa: function(i, btoa) {
      let _i = A.g(i);

      let _btoa = this.s(btoa);
      return this.ms(_i.btoa(_btoa));
    },

    window_atob: function(i, atob) {
      let _i = A.g(i);

      let _atob = this.s(atob);
      return this.ms(_i.atob(_atob));
    },

    window_set_timeout: function(i, handler, timeout) {
      let _i = A.g(i);

      let _handler = A.g(handler);
      let _timeout = timeout;
      return A.a(_i.setTimeout(_handler, _timeout));
    },

    window_clear_timeout: function(i, handle) {
      let _i = A.g(i);

      let _handle = handle;
      _i.clearTimeout(_handle);
    },

    window_set_interval: function(i, handler, timeout) {
      let _i = A.g(i);

      let _handler = A.g(handler);
      let _timeout = timeout;
      return A.a(_i.setInterval(_handler, _timeout));
    },

    window_clear_interval: function(i, handle) {
      let _i = A.g(i);

      let _handle = handle;
      _i.clearInterval(_handle);
    },

    window_create_image_bitmap: function(i, aImage, aSx, aSy, aSw, aSh) {
      let _i = A.g(i);

      let _aImage = A.g(aImage);
      let _aSx = aSx;
      let _aSy = aSy;
      let _aSw = aSw;
      let _aSh = aSh;
      return A.a(_i.createImageBitmap(_aImage, _aSx, _aSy, _aSw, _aSh));
    }
  };
  return webidl;
}

export default createWebIDLContext;
